use engine::TileType;

/// A room template defining layout, spawn points, and entry points.
pub struct RoomTemplate {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
    pub spawn_points: Vec<SpawnPoint>,
    pub entry_points: Vec<EntryPoint>,
    pub player_spawn: Option<(usize, usize)>,
    pub room_type: RoomType,
}

/// The purpose of a room in the dungeon.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoomType {
    Combat,
    Treasure,
    Shop,
    Boss,
    Start,
    Exit,
    Corridor,
}

/// A location where enemies can spawn within a room.
pub struct SpawnPoint {
    pub x: usize,
    pub y: usize,
    /// Spawn wave group (0 = immediate, 1 = wave 2, etc.)
    #[allow(dead_code)] // Used for group-based wave spawning in encounters
    pub group: u8,
}

/// A doorway connecting this room to another.
pub struct EntryPoint {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

/// Cardinal direction for entry points.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Parse a template string into a `RoomTemplate`.
///
/// Template characters:
/// - `W` = Wall
/// - `.` = Floor
/// - `D` = Door (closed)
/// - `S` = Spawn point (floor tile underneath, group 0)
/// - `P` = Player spawn (floor tile underneath)
/// - `E` = Exit/stairs (floor tile underneath)
///
/// Spawn groups can be specified with digits `0`-`9` instead of `S` for
/// explicit wave groups. `S` defaults to group 0.
///
/// Entry points are automatically detected at door positions based on
/// which wall edge they occupy (North/South/East/West).
pub fn parse_template(layout: &[&str], room_type: RoomType) -> RoomTemplate {
    let height = layout.len();
    assert!(height > 0, "Room template must have at least one row");
    let width = layout[0].len();
    assert!(width > 0, "Room template must have at least one column");

    let mut tiles = vec![TileType::Floor; width * height];
    let mut spawn_points = Vec::new();
    let mut entry_points = Vec::new();
    let mut player_spawn = None;

    for (y, row) in layout.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let idx = y * width + x;
            match ch {
                'W' => tiles[idx] = TileType::Wall,
                '.' => tiles[idx] = TileType::Floor,
                'D' => {
                    tiles[idx] = TileType::DoorClosed;
                    let direction = infer_direction(x, y, width, height);
                    entry_points.push(EntryPoint { x, y, direction });
                }
                'S' => {
                    tiles[idx] = TileType::Floor;
                    spawn_points.push(SpawnPoint { x, y, group: 0 });
                }
                '0'..='9' => {
                    tiles[idx] = TileType::Floor;
                    let group = (ch as u8) - b'0';
                    spawn_points.push(SpawnPoint { x, y, group });
                }
                'P' => {
                    tiles[idx] = TileType::Floor;
                    player_spawn = Some((x, y));
                }
                'E' => {
                    tiles[idx] = TileType::Floor;
                    // Exit marker — stored as player_spawn for Exit rooms
                    if room_type == RoomType::Exit {
                        player_spawn = Some((x, y));
                    }
                }
                _ => tiles[idx] = TileType::Floor,
            }
        }
    }

    // Apply WallTop: any Wall tile directly above a non-solid tile
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if tiles[idx] == TileType::Wall && y + 1 < height {
                let below = tiles[(y + 1) * width + x];
                if !matches!(
                    below,
                    TileType::Wall | TileType::WallTop | TileType::DoorClosed
                ) {
                    tiles[idx] = TileType::WallTop;
                }
            }
        }
    }

    RoomTemplate {
        width,
        height,
        tiles,
        spawn_points,
        entry_points,
        player_spawn,
        room_type,
    }
}

/// Infer the direction of a door based on its position relative to the room edges.
fn infer_direction(x: usize, y: usize, width: usize, height: usize) -> Direction {
    if y == 0 {
        Direction::North
    } else if y == height - 1 {
        Direction::South
    } else if x == 0 {
        Direction::West
    } else if x == width - 1 {
        Direction::East
    } else {
        // Interior door — pick direction based on nearest edge
        let dist_n = y;
        let dist_s = height - 1 - y;
        let dist_w = x;
        let dist_e = width - 1 - x;
        let min = dist_n.min(dist_s).min(dist_w).min(dist_e);
        if min == dist_n {
            Direction::North
        } else if min == dist_s {
            Direction::South
        } else if min == dist_w {
            Direction::West
        } else {
            Direction::East
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_room() {
        let layout = &[
            "WWWWW",
            "W...W",
            "W.P.W",
            "W...W",
            "WWDWW",
        ];
        let room = parse_template(layout, RoomType::Start);
        assert_eq!(room.width, 5);
        assert_eq!(room.height, 5);
        assert_eq!(room.player_spawn, Some((2, 2)));
        assert_eq!(room.entry_points.len(), 1);
        assert_eq!(room.entry_points[0].direction, Direction::South);
        // Tile at (2,0): top row, middle — below is Floor at (2,1), so becomes WallTop
        assert_eq!(room.tiles[2], TileType::WallTop);
        // Tile at (0,0): corner — below is Wall at (0,1), stays Wall
        assert_eq!(room.tiles[0], TileType::Wall);
    }

    #[test]
    fn spawn_groups() {
        let layout = &[
            "WWWWW",
            "W0.1W",
            "W.S.W",
            "WWWWW",
        ];
        let room = parse_template(layout, RoomType::Combat);
        assert_eq!(room.spawn_points.len(), 3);
        assert_eq!(room.spawn_points[0].group, 0);
        assert_eq!(room.spawn_points[1].group, 1);
        assert_eq!(room.spawn_points[2].group, 0); // 'S' defaults to 0
    }

    #[test]
    fn door_directions() {
        let layout = &[
            "WWDWW",
            "D...D",
            "W...W",
            "WWDWW",
        ];
        let room = parse_template(layout, RoomType::Combat);
        assert_eq!(room.entry_points.len(), 4);

        let dirs: Vec<Direction> = room.entry_points.iter().map(|e| e.direction).collect();
        assert!(dirs.contains(&Direction::North));
        assert!(dirs.contains(&Direction::South));
        assert!(dirs.contains(&Direction::East));
        assert!(dirs.contains(&Direction::West));
    }
}
