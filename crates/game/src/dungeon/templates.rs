use super::room_template::{parse_template, RoomTemplate, RoomType};

/// 1. Start Room (16x12): Open room, entry south. Player spawns center.
pub fn start_room() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWWWWWWWWWWWWWWW",
        "W..............W",
        "W..............W",
        "W..............W",
        "W..............W",
        "W......P.......W",
        "W..............W",
        "W..............W",
        "W..............W",
        "W..............W",
        "W..............W",
        "WWWWWWWDWWWWWWWW",
    ];
    parse_template(layout, RoomType::Start)
}

/// 2. Arena (20x14): Open combat room. 4 entries (one per side). 6 spawn points.
pub fn arena() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWWWWWWWWDWWWWWWWWWW",
        "W..................W",
        "W..S...........S..W",
        "W..................W",
        "W..................W",
        "W......S..S........W",
        "D..................D",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..S...........S..W",
        "W..................W",
        "W..................W",
        "WWWWWWWWWDWWWWWWWWWW",
    ];
    parse_template(layout, RoomType::Combat)
}

/// 3. Pillared Hall (20x14): Combat room with 4 interior 2x2 pillars.
pub fn pillared_hall() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWWWWWWWWDWWWWWWWWWW",
        "W..................W",
        "W..S..........S...W",
        "W....WW....WW.....W",
        "W....WW....WW.....W",
        "W..S..........S...W",
        "D..........S......D",
        "W..............S..W",
        "W..................W",
        "W....WW....WW.....W",
        "W....WW....WW.....W",
        "W..S..........S...W",
        "W..................W",
        "WWWWWWWWWDWWWWWWWWWW",
    ];
    parse_template(layout, RoomType::Combat)
}

/// 4. Corridor-H (18x6): Horizontal corridor. East+west entries. 2 spawn points.
pub fn corridor_h() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWWWWWWWWWWWWWWWWW",
        "W................W",
        "D.....S....S.....D",
        "D................D",
        "W................W",
        "WWWWWWWWWWWWWWWWWW",
    ];
    parse_template(layout, RoomType::Corridor)
}

/// 5. Corridor-V (6x18): Vertical corridor. North+south entries. 2 spawn points.
pub fn corridor_v() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWDDWW",
        "W....W",
        "W....W",
        "W....W",
        "W..S.W",
        "W....W",
        "W....W",
        "W....W",
        "W....W",
        "W....W",
        "W....W",
        "W....W",
        "W....W",
        "W.S..W",
        "W....W",
        "W....W",
        "W....W",
        "WWDDWW",
    ];
    parse_template(layout, RoomType::Corridor)
}

/// 6. L-Shape (16x14): L-shaped room with walls creating the turn. 2 entries, 4 spawn points.
pub fn l_shape() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWWWWWWWWWWWWWWW",
        "W..........WWWWW",
        "W..S.......WWWWW",
        "W..........WWWWW",
        "W..........WWWWW",
        "W..............W",
        "D......S...S...W",
        "W..............W",
        "W..............W",
        "W..............W",
        "W......S.......W",
        "W..............W",
        "W..............W",
        "WWWWWWWWDWWWWWWW",
    ];
    parse_template(layout, RoomType::Combat)
}

/// 7. Treasure Vault (10x8): Single entry. Chest spawn in center.
pub fn treasure_vault() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWWWWWWWWW",
        "W........W",
        "W........W",
        "W...SS...W",
        "W...SS...W",
        "W........W",
        "W........W",
        "WWWWDWWWWW",
    ];
    parse_template(layout, RoomType::Treasure)
}

/// 8. Boss Arena (26x20): Large open arena. Single entry south. Boss spawn center.
pub fn boss_arena() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWWWWWWWWWWWWWWWWWWWWWWWWW",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W............S...........W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "W........................W",
        "WWWWWWWWWWWWDWWWWWWWWWWWWW",
    ];
    parse_template(layout, RoomType::Boss)
}

/// 9. Shop (16x12): Counter-like wall structure. Single entry.
pub fn shop() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWWWWWWWWWWWWWWW",
        "W..............W",
        "W..............W",
        "W..WWWWWWWWWW..W",
        "W..W........W..W",
        "W..............W",
        "W..............W",
        "W..S...S...S...W",
        "W..............W",
        "W..............W",
        "W..............W",
        "WWWWWWWDWWWWWWWW",
    ];
    parse_template(layout, RoomType::Shop)
}

/// 10. Exit Room (12x10): Stairs-down in center. Single entry.
pub fn exit_room() -> RoomTemplate {
    #[rustfmt::skip]
    let layout = &[
        "WWWWWWWWWWWW",
        "W..........W",
        "W..........W",
        "W..........W",
        "W.....E....W",
        "W..........W",
        "W..........W",
        "W..........W",
        "W..........W",
        "WWWWWDWWWWWW",
    ];
    parse_template(layout, RoomType::Exit)
}

/// Returns all available room templates.
#[allow(dead_code)] // Used in tests
pub fn all_templates() -> Vec<RoomTemplate> {
    vec![
        start_room(),
        arena(),
        pillared_hall(),
        corridor_h(),
        corridor_v(),
        l_shape(),
        treasure_vault(),
        boss_arena(),
        shop(),
        exit_room(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_templates_parse_without_panic() {
        let templates = all_templates();
        assert_eq!(templates.len(), 10);
    }

    #[test]
    fn start_room_has_player_spawn() {
        let room = start_room();
        assert_eq!(room.room_type, RoomType::Start);
        assert!(room.player_spawn.is_some());
        assert_eq!(room.entry_points.len(), 1);
    }

    #[test]
    fn arena_has_four_entries() {
        let room = arena();
        assert_eq!(room.room_type, RoomType::Combat);
        assert_eq!(room.entry_points.len(), 4);
        assert_eq!(room.spawn_points.len(), 6);
    }

    #[test]
    fn pillared_hall_dimensions() {
        let room = pillared_hall();
        assert_eq!(room.width, 20);
        assert_eq!(room.height, 14);
        assert_eq!(room.entry_points.len(), 4);
        assert_eq!(room.spawn_points.len(), 8);
    }

    #[test]
    fn corridor_h_dimensions() {
        let room = corridor_h();
        assert_eq!(room.width, 18);
        assert_eq!(room.height, 6);
        assert_eq!(room.spawn_points.len(), 2);
    }

    #[test]
    fn corridor_v_dimensions() {
        let room = corridor_v();
        assert_eq!(room.width, 6);
        assert_eq!(room.height, 18);
        assert_eq!(room.spawn_points.len(), 2);
    }

    #[test]
    fn boss_arena_is_large() {
        let room = boss_arena();
        assert_eq!(room.width, 26);
        assert_eq!(room.height, 20);
        assert_eq!(room.room_type, RoomType::Boss);
        assert_eq!(room.entry_points.len(), 1);
    }

    #[test]
    fn exit_room_has_exit_marker() {
        let room = exit_room();
        assert_eq!(room.room_type, RoomType::Exit);
        // Exit marker stored as player_spawn for Exit rooms
        assert!(room.player_spawn.is_some());
    }

    #[test]
    fn template_tile_counts_match_dimensions() {
        for template in all_templates() {
            assert_eq!(
                template.tiles.len(),
                template.width * template.height,
                "Tile count mismatch for {:?} room ({}x{})",
                template.room_type,
                template.width,
                template.height,
            );
        }
    }
}
