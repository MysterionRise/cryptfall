use super::room_template::{Direction, RoomTemplate, RoomType};
use super::templates;

/// A fully generated floor layout.
pub struct FloorLayout {
    pub rooms: Vec<PlacedRoom>,
    pub connections: Vec<(usize, usize)>,
    #[allow(dead_code)] // Used for floor-based scoring in Phase 4
    pub floor_number: u32,
}

/// A room placed at a specific grid position.
pub struct PlacedRoom {
    pub template: RoomTemplate,
    pub grid_x: i32,
    pub grid_y: i32,
    pub room_type: RoomType,
    pub cleared: bool,
    pub discovered: bool,
}

/// Floor difficulty configuration.
pub struct FloorConfig {
    pub floor_number: u32,
    pub min_rooms: usize,
    pub max_rooms: usize,
    #[allow(dead_code)] // Used for encounter scaling in Phase 4
    pub enemy_count_mult: f32,
    #[allow(dead_code)] // Used for encounter scaling in Phase 4
    pub enemy_hp_mult: f32,
}

impl FloorConfig {
    pub fn from_floor(floor_number: u32) -> Self {
        let f = floor_number.saturating_sub(1) as f32;
        Self {
            floor_number,
            min_rooms: (6 + floor_number.saturating_sub(1) as usize).min(12),
            max_rooms: (10 + floor_number.saturating_sub(1) as usize).min(15),
            enemy_count_mult: 1.0 + f * 0.15,
            enemy_hp_mult: 1.0 + f * 0.1,
        }
    }
}

/// Simple seeded PRNG (xorshift64).
struct FloorRng {
    state: u64,
}

impl FloorRng {
    fn new(seed: u64) -> Self {
        // Ensure state is never zero (xorshift requires nonzero)
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn range(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }
        min + (self.next() as usize % (max - min + 1))
    }

    #[allow(dead_code)] // Utility for future template selection variety
    fn choose<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        let idx = self.next() as usize % items.len();
        &items[idx]
    }
}

/// Returns the opposite direction.
fn opposite(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

/// Returns the grid offset for a direction.
fn direction_offset(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
    }
}

/// Check whether a template has an entry point in the given direction.
fn template_has_direction(template: &RoomTemplate, dir: Direction) -> bool {
    template.entry_points.iter().any(|ep| ep.direction == dir)
}

/// Get all directions a template has entry points for.
fn template_directions(template: &RoomTemplate) -> Vec<Direction> {
    let mut dirs = Vec::new();
    for ep in &template.entry_points {
        if !dirs.contains(&ep.direction) {
            dirs.push(ep.direction);
        }
    }
    dirs
}

/// Select a combat template that has an entry point in the required direction.
fn pick_combat_template(rng: &mut FloorRng, needed_dir: Direction) -> RoomTemplate {
    // Combat templates: arena, pillared_hall, l_shape
    // All three have entries on multiple sides, but we must verify the needed direction.
    let candidates: Vec<fn() -> RoomTemplate> =
        vec![templates::arena, templates::pillared_hall, templates::l_shape];

    // Try up to 10 times to pick a compatible template
    for _ in 0..10 {
        let idx = rng.next() as usize % candidates.len();
        let template = candidates[idx]();
        if template_has_direction(&template, needed_dir) {
            return template;
        }
    }

    // Fallback: arena always has all 4 directions
    templates::arena()
}

/// Select a corridor template based on the connection direction.
fn pick_corridor_template(dir: Direction) -> RoomTemplate {
    match dir {
        Direction::East | Direction::West => templates::corridor_h(),
        Direction::North | Direction::South => templates::corridor_v(),
    }
}

/// Generate a procedural floor layout.
///
/// The algorithm places rooms on an abstract grid using a Binding of Isaac-style
/// approach: start at (0,0), then iteratively grow by placing rooms adjacent to
/// open connections. Special rooms (boss, exit, treasure, shop) are assigned to
/// dead-end positions after the main layout is generated.
pub fn generate_floor(floor_number: u32, seed: u64) -> FloorLayout {
    let config = FloorConfig::from_floor(floor_number);

    // Retry with different seeds if generation fails
    for attempt in 0..20u64 {
        let adjusted_seed = seed.wrapping_add(attempt.wrapping_mul(7919));
        if let Some(layout) = try_generate(&config, adjusted_seed) {
            return layout;
        }
    }

    // Ultimate fallback: minimal valid floor
    fallback_floor(floor_number)
}

/// Attempt to generate a floor. Returns None if constraints can't be met.
fn try_generate(config: &FloorConfig, seed: u64) -> Option<FloorLayout> {
    let mut rng = FloorRng::new(seed);
    let target_rooms = rng.range(config.min_rooms, config.max_rooms);

    let mut rooms: Vec<PlacedRoom> = Vec::new();
    let mut connections: Vec<(usize, usize)> = Vec::new();
    // Track occupied grid positions: (grid_x, grid_y) -> room index
    let mut occupied: Vec<(i32, i32, usize)> = Vec::new();
    // Open connections: (room_index, direction) pairs where we can attach new rooms
    let mut open_connections: Vec<(usize, Direction)> = Vec::new();

    // 1. Place start room at (0, 0)
    let start_template = templates::start_room();
    let start_dirs = template_directions(&start_template);
    rooms.push(PlacedRoom {
        template: start_template,
        grid_x: 0,
        grid_y: 0,
        room_type: RoomType::Start,
        cleared: true,
        discovered: true,
    });
    occupied.push((0, 0, 0));

    for dir in start_dirs {
        open_connections.push((0, dir));
    }

    // 2. Grow the floor by placing rooms at open connections
    let mut stall_count = 0;
    while rooms.len() < target_rooms && !open_connections.is_empty() && stall_count < 100 {
        // Pick a random open connection
        let conn_idx = rng.next() as usize % open_connections.len();
        let (source_room, dir) = open_connections[conn_idx];

        let (dx, dy) = direction_offset(dir);
        let new_x = rooms[source_room].grid_x + dx;
        let new_y = rooms[source_room].grid_y + dy;

        // Check if position is already occupied
        if occupied.iter().any(|&(x, y, _)| x == new_x && y == new_y) {
            // Remove this connection since it can't be used
            open_connections.swap_remove(conn_idx);
            stall_count += 1;
            continue;
        }

        // The new room needs an entry in the opposite direction to connect back
        let needed_dir = opposite(dir);

        // Pick a template (mostly combat rooms, occasionally corridors)
        let template = if rng.next().is_multiple_of(5) {
            // 20% chance of corridor
            let corridor = pick_corridor_template(dir);
            if template_has_direction(&corridor, needed_dir) {
                corridor
            } else {
                pick_combat_template(&mut rng, needed_dir)
            }
        } else {
            pick_combat_template(&mut rng, needed_dir)
        };

        let room_type = template.room_type;
        let new_dirs = template_directions(&template);
        let new_room_idx = rooms.len();

        rooms.push(PlacedRoom {
            template,
            grid_x: new_x,
            grid_y: new_y,
            room_type,
            cleared: false,
            discovered: false,
        });
        occupied.push((new_x, new_y, new_room_idx));

        // Connect the two rooms
        connections.push((source_room, new_room_idx));

        // Remove the used connection from source
        open_connections.swap_remove(conn_idx);

        // Add new room's remaining open directions (excluding the one connecting back)
        for d in new_dirs {
            if d != needed_dir {
                open_connections.push((new_room_idx, d));
            }
        }

        stall_count = 0;
    }

    // Check minimum room count
    if rooms.len() < config.min_rooms {
        return None;
    }

    // 3. Identify dead ends (rooms with only one connection)
    let mut connection_count = vec![0usize; rooms.len()];
    for &(a, b) in &connections {
        connection_count[a] += 1;
        connection_count[b] += 1;
    }

    let mut dead_ends: Vec<usize> = (0..rooms.len())
        .filter(|&i| connection_count[i] == 1 && rooms[i].room_type != RoomType::Start)
        .collect();

    // Sort dead ends by distance from start (BFS distance)
    let distances = bfs_distances(&connections, rooms.len(), 0);
    dead_ends.sort_by(|&a, &b| distances[b].cmp(&distances[a])); // furthest first

    // 4. Assign special rooms to dead ends
    // Boss: furthest dead end
    if let Some(&boss_idx) = dead_ends.first() {
        let boss_template = templates::boss_arena();
        rooms[boss_idx].template = boss_template;
        rooms[boss_idx].room_type = RoomType::Boss;
    }

    // Exit: second furthest dead end
    if dead_ends.len() > 1 {
        let exit_idx = dead_ends[1];
        let exit_template = templates::exit_room();
        rooms[exit_idx].template = exit_template;
        rooms[exit_idx].room_type = RoomType::Exit;
    }

    // Treasure: third dead end if available
    if dead_ends.len() > 2 {
        let treasure_idx = dead_ends[2];
        let treasure_template = templates::treasure_vault();
        rooms[treasure_idx].template = treasure_template;
        rooms[treasure_idx].room_type = RoomType::Treasure;
    }

    // Shop: fourth dead end if available
    if dead_ends.len() > 3 {
        let shop_idx = dead_ends[3];
        let shop_template = templates::shop();
        rooms[shop_idx].template = shop_template;
        rooms[shop_idx].room_type = RoomType::Shop;
    }

    // If no dead ends found for boss/exit, we need at least those two
    let has_boss = rooms.iter().any(|r| r.room_type == RoomType::Boss);
    let has_exit = rooms.iter().any(|r| r.room_type == RoomType::Exit);
    if !has_boss || !has_exit {
        return None;
    }

    // 5. Validate all rooms reachable
    if !all_reachable(&connections, rooms.len(), 0) {
        return None;
    }

    Some(FloorLayout {
        rooms,
        connections,
        floor_number: config.floor_number,
    })
}

/// BFS from `start` to compute distances to all rooms.
fn bfs_distances(connections: &[(usize, usize)], num_rooms: usize, start: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; num_rooms];
    dist[start] = 0;

    // Build adjacency list
    let mut adj = vec![Vec::new(); num_rooms];
    for &(a, b) in connections {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adj[node] {
            if dist[neighbor] == usize::MAX {
                dist[neighbor] = dist[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    dist
}

/// Check if all rooms are reachable from `start` via BFS.
fn all_reachable(connections: &[(usize, usize)], num_rooms: usize, start: usize) -> bool {
    let distances = bfs_distances(connections, num_rooms, start);
    distances.iter().all(|&d| d != usize::MAX)
}

/// Minimal fallback floor: start -> combat -> boss -> exit (linear).
fn fallback_floor(floor_number: u32) -> FloorLayout {
    let rooms = vec![
        PlacedRoom {
            template: templates::start_room(),
            grid_x: 0,
            grid_y: 0,
            room_type: RoomType::Start,
            cleared: true,
            discovered: true,
        },
        PlacedRoom {
            template: templates::arena(),
            grid_x: 0,
            grid_y: 1,
            room_type: RoomType::Combat,
            cleared: false,
            discovered: false,
        },
        PlacedRoom {
            template: templates::arena(),
            grid_x: 0,
            grid_y: 2,
            room_type: RoomType::Combat,
            cleared: false,
            discovered: false,
        },
        PlacedRoom {
            template: templates::boss_arena(),
            grid_x: 0,
            grid_y: 3,
            room_type: RoomType::Boss,
            cleared: false,
            discovered: false,
        },
        PlacedRoom {
            template: templates::exit_room(),
            grid_x: 0,
            grid_y: 4,
            room_type: RoomType::Exit,
            cleared: false,
            discovered: false,
        },
    ];
    let connections = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
    FloorLayout {
        rooms,
        connections,
        floor_number,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_room_count_in_range() {
        let layout = generate_floor(1, 12345);
        let config = FloorConfig::from_floor(1);
        assert!(
            layout.rooms.len() >= config.min_rooms,
            "Too few rooms: {} < {}",
            layout.rooms.len(),
            config.min_rooms
        );
        assert!(
            layout.rooms.len() <= config.max_rooms,
            "Too many rooms: {} > {}",
            layout.rooms.len(),
            config.max_rooms
        );
    }

    #[test]
    fn start_room_at_origin() {
        let layout = generate_floor(1, 42);
        assert_eq!(layout.rooms[0].room_type, RoomType::Start);
        assert_eq!(layout.rooms[0].grid_x, 0);
        assert_eq!(layout.rooms[0].grid_y, 0);
        assert!(layout.rooms[0].discovered);
        assert!(layout.rooms[0].cleared);
    }

    #[test]
    fn boss_and_exit_exist() {
        let layout = generate_floor(1, 99999);
        let has_boss = layout.rooms.iter().any(|r| r.room_type == RoomType::Boss);
        let has_exit = layout.rooms.iter().any(|r| r.room_type == RoomType::Exit);
        assert!(has_boss, "Floor must have a boss room");
        assert!(has_exit, "Floor must have an exit room");
    }

    #[test]
    fn no_duplicate_grid_positions() {
        let layout = generate_floor(1, 77777);
        let positions: Vec<(i32, i32)> = layout
            .rooms
            .iter()
            .map(|r| (r.grid_x, r.grid_y))
            .collect();
        for (i, pos) in positions.iter().enumerate() {
            for (j, other) in positions.iter().enumerate() {
                if i != j {
                    assert_ne!(
                        pos, other,
                        "Rooms {} and {} share grid position {:?}",
                        i, j, pos
                    );
                }
            }
        }
    }

    #[test]
    fn all_rooms_reachable_from_start() {
        let layout = generate_floor(1, 55555);
        assert!(
            all_reachable(&layout.connections, layout.rooms.len(), 0),
            "All rooms must be reachable from the start room"
        );
    }

    #[test]
    fn deterministic_with_same_seed() {
        let layout_a = generate_floor(1, 31337);
        let layout_b = generate_floor(1, 31337);

        assert_eq!(layout_a.rooms.len(), layout_b.rooms.len());
        assert_eq!(layout_a.connections.len(), layout_b.connections.len());

        for (a, b) in layout_a.rooms.iter().zip(layout_b.rooms.iter()) {
            assert_eq!(a.grid_x, b.grid_x);
            assert_eq!(a.grid_y, b.grid_y);
            assert_eq!(a.room_type, b.room_type);
        }

        for (a, b) in layout_a.connections.iter().zip(layout_b.connections.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn different_seeds_produce_different_layouts() {
        let layout_a = generate_floor(1, 100);
        let layout_b = generate_floor(1, 200);

        // At minimum, positions or room count should differ
        let positions_a: Vec<(i32, i32)> = layout_a
            .rooms
            .iter()
            .map(|r| (r.grid_x, r.grid_y))
            .collect();
        let positions_b: Vec<(i32, i32)> = layout_b
            .rooms
            .iter()
            .map(|r| (r.grid_x, r.grid_y))
            .collect();
        // Very unlikely for two random seeds to produce identical layouts
        assert!(
            positions_a != positions_b || layout_a.rooms.len() != layout_b.rooms.len(),
            "Different seeds should generally produce different layouts"
        );
    }

    #[test]
    fn higher_floors_have_more_rooms() {
        let config_1 = FloorConfig::from_floor(1);
        let config_5 = FloorConfig::from_floor(5);
        assert!(config_5.min_rooms > config_1.min_rooms);
        assert!(config_5.max_rooms > config_1.max_rooms);
        assert!(config_5.enemy_count_mult > config_1.enemy_count_mult);
        assert!(config_5.enemy_hp_mult > config_1.enemy_hp_mult);
    }

    #[test]
    fn floor_config_caps() {
        let config = FloorConfig::from_floor(100);
        assert_eq!(config.min_rooms, 12);
        assert_eq!(config.max_rooms, 15);
    }

    #[test]
    fn multiple_seeds_all_valid() {
        // Generate floors with many different seeds to stress test
        for seed in 0..50u64 {
            let layout = generate_floor(1, seed);
            assert!(layout.rooms.len() >= 5, "Seed {} produced too few rooms", seed);
            assert!(
                layout.rooms.iter().any(|r| r.room_type == RoomType::Boss),
                "Seed {} missing boss room",
                seed
            );
            assert!(
                layout.rooms.iter().any(|r| r.room_type == RoomType::Exit),
                "Seed {} missing exit room",
                seed
            );
            assert!(
                all_reachable(&layout.connections, layout.rooms.len(), 0),
                "Seed {} has unreachable rooms",
                seed
            );
        }
    }
}
