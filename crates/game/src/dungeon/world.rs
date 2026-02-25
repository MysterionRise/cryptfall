use engine::tilemap::{TileMap, TileType, TILE_SIZE};

use super::floor_gen::{generate_floor, FloorLayout, PlacedRoom};
use super::room_template::{Direction, RoomType};

/// Duration of the fade-out phase in seconds.
const FADE_OUT_DURATION: f32 = 0.3;
/// Duration of the fade-in phase in seconds.
const FADE_IN_DURATION: f32 = 0.3;

/// Manages the active dungeon: current floor, room transitions, and per-room state.
pub struct DungeonWorld {
    pub floor: FloorLayout,
    pub current_room_index: usize,
    pub transition: Option<RoomTransition>,
    pub floor_number: u32,
    seed: u64,
}

pub struct RoomTransition {
    pub to_room: usize,
    pub direction: Direction,
    pub phase: TransitionPhase,
    pub timer: f32,
}

#[derive(PartialEq)]
pub enum TransitionPhase {
    FadeOut,
    Load,
    FadeIn,
}

pub enum TransitionEvent {
    SwapRoom,
    Complete,
}

impl DungeonWorld {
    pub fn new(floor_number: u32, seed: u64) -> Self {
        let floor = generate_floor(floor_number, seed);
        Self {
            floor,
            current_room_index: 0,
            transition: None,
            floor_number,
            seed,
        }
    }

    pub fn current_room(&self) -> &PlacedRoom {
        &self.floor.rooms[self.current_room_index]
    }

    /// Build a TileMap from the current room's template.
    pub fn build_tilemap(&self) -> TileMap {
        self.build_tilemap_for(self.current_room_index)
    }

    /// Build a TileMap for a specific room index.
    fn build_tilemap_for(&self, room_index: usize) -> TileMap {
        let room = &self.floor.rooms[room_index];
        let template = &room.template;
        let mut tilemap = TileMap::new(template.width, template.height);

        for y in 0..template.height {
            for x in 0..template.width {
                let tile = template.tiles[y * template.width + x];
                tilemap.set(x, y, tile);
            }
        }

        // If room is cleared or is the start room, open all doors
        if room.cleared || room.room_type == RoomType::Start {
            set_doors(&mut tilemap, true);
        }

        tilemap
    }

    /// Begin a room transition.
    pub fn start_transition(&mut self, to_room: usize, direction: Direction) {
        self.transition = Some(RoomTransition {
            to_room,
            direction,
            phase: TransitionPhase::FadeOut,
            timer: 0.0,
        });
    }

    /// Tick the transition timer. Returns events when phase boundaries are crossed.
    pub fn update_transition(&mut self, dt: f32) -> Option<TransitionEvent> {
        let transition = self.transition.as_mut()?;

        transition.timer += dt;

        match transition.phase {
            TransitionPhase::FadeOut => {
                if transition.timer >= FADE_OUT_DURATION {
                    transition.phase = TransitionPhase::Load;
                    transition.timer = 0.0;
                    return Some(TransitionEvent::SwapRoom);
                }
            }
            TransitionPhase::Load => {
                // Instant swap phase — immediately move to fade in
                transition.phase = TransitionPhase::FadeIn;
                transition.timer = 0.0;
            }
            TransitionPhase::FadeIn => {
                if transition.timer >= FADE_IN_DURATION {
                    self.transition = None;
                    return Some(TransitionEvent::Complete);
                }
            }
        }

        None
    }

    /// Get the transition overlay opacity (0.0 = fully visible, 1.0 = fully dark).
    pub fn transition_opacity(&self) -> f32 {
        match &self.transition {
            Some(t) => match t.phase {
                TransitionPhase::FadeOut => (t.timer / FADE_OUT_DURATION).min(1.0),
                TransitionPhase::Load => 1.0,
                TransitionPhase::FadeIn => 1.0 - (t.timer / FADE_IN_DURATION).min(1.0),
            },
            None => 0.0,
        }
    }

    /// Check if the player (center position) is overlapping an open door tile
    /// that connects to another room. Returns (connected_room_index, direction).
    pub fn check_door_collision(
        &self,
        player_x: f32,
        player_y: f32,
        tilemap: &TileMap,
    ) -> Option<(usize, Direction)> {
        let ts = TILE_SIZE as f32;
        let tx = (player_x / ts) as usize;
        let ty = (player_y / ts) as usize;

        // Check the tile the player center is on
        if tilemap.get(tx, ty) != TileType::DoorOpen {
            return None;
        }

        // Find which entry point this door corresponds to
        let room = self.current_room();
        for ep in &room.template.entry_points {
            if ep.x == tx && ep.y == ty {
                // Find the connected room via this direction
                if let Some(connected) = self.find_connected_room(self.current_room_index, ep.direction) {
                    return Some((connected, ep.direction));
                }
            }
        }

        // Also check adjacent door tiles (doors can span 2 tiles)
        for ep in &room.template.entry_points {
            let dx = (ep.x as i32 - tx as i32).abs();
            let dy = (ep.y as i32 - ty as i32).abs();
            if dx <= 1 && dy <= 1 && (dx + dy) <= 1 {
                if let Some(connected) = self.find_connected_room(self.current_room_index, ep.direction) {
                    return Some((connected, ep.direction));
                }
            }
        }

        None
    }

    /// Find the room connected to `room_index` in the given direction.
    /// Uses grid positions to determine which connection matches the direction.
    fn find_connected_room(&self, room_index: usize, direction: Direction) -> Option<usize> {
        let room = &self.floor.rooms[room_index];
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };
        let target_x = room.grid_x + dx;
        let target_y = room.grid_y + dy;

        for &(a, b) in &self.floor.connections {
            let other = if a == room_index {
                b
            } else if b == room_index {
                a
            } else {
                continue;
            };
            let other_room = &self.floor.rooms[other];
            if other_room.grid_x == target_x && other_room.grid_y == target_y {
                return Some(other);
            }
        }
        None
    }

    /// Mark a room as cleared.
    pub fn mark_room_cleared(&mut self, room_index: usize) {
        self.floor.rooms[room_index].cleared = true;
    }

    /// Mark a room as discovered.
    pub fn mark_room_discovered(&mut self, room_index: usize) {
        self.floor.rooms[room_index].discovered = true;
    }

    /// Get the player spawn position in pixel coords for the current room.
    /// If entering from a direction, find the matching entry point.
    /// Otherwise use the room's player_spawn or center.
    pub fn player_spawn_position(&self, from_direction: Option<Direction>) -> (f32, f32) {
        let room = self.current_room();
        let ts = TILE_SIZE as f32;

        if let Some(dir) = from_direction {
            // Player enters from `dir`, so find the entry point on the opposite side
            let enter_from = opposite(dir);
            for ep in &room.template.entry_points {
                if ep.direction == enter_from {
                    // Offset player slightly inside the room from the door
                    let (offset_x, offset_y) = match enter_from {
                        Direction::North => (0.0, 1.5 * ts),
                        Direction::South => (0.0, -1.5 * ts),
                        Direction::East => (-1.5 * ts, 0.0),
                        Direction::West => (1.5 * ts, 0.0),
                    };
                    return (
                        ep.x as f32 * ts + offset_x - 5.0,
                        ep.y as f32 * ts + offset_y - 7.0,
                    );
                }
            }
        }

        // Fallback: use player_spawn from template
        if let Some((px, py)) = room.template.player_spawn {
            return (px as f32 * ts - 5.0, py as f32 * ts - 7.0);
        }

        // Ultimate fallback: room center
        let cx = (room.template.width as f32 * ts) / 2.0;
        let cy = (room.template.height as f32 * ts) / 2.0;
        (cx - 5.0, cy - 7.0)
    }

    /// Regenerate the floor (for restart on death).
    pub fn reset(&mut self) {
        self.floor = generate_floor(self.floor_number, self.seed);
        self.current_room_index = 0;
        self.transition = None;
    }

    /// Advance to the next floor.
    pub fn next_floor(&mut self) {
        self.floor_number += 1;
        self.seed = self.seed.wrapping_add(12345);
        self.floor = generate_floor(self.floor_number, self.seed);
        self.current_room_index = 0;
        self.transition = None;
    }

    /// Swap to the target room (called during Load phase).
    pub fn swap_to_room(&mut self, room_index: usize) {
        self.current_room_index = room_index;
        self.mark_room_discovered(room_index);
    }

    /// Check if current room is the exit room.
    pub fn is_exit_room(&self) -> bool {
        self.current_room().room_type == RoomType::Exit
    }
}

/// Toggle all door tiles in a tilemap between open and closed.
pub fn set_doors(tilemap: &mut TileMap, open: bool) {
    let (from, to) = if open {
        (TileType::DoorClosed, TileType::DoorOpen)
    } else {
        (TileType::DoorOpen, TileType::DoorClosed)
    };

    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            if tilemap.get(x, y) == from {
                tilemap.set(x, y, to);
            }
        }
    }
}

fn opposite(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}
