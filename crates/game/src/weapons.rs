#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum WeaponId {
    Sword,
    Spear,
    Daggers,
}

pub struct WeaponDef {
    pub id: WeaponId,
    pub name: &'static str,
    pub base_damage: i32,
    pub attack_cooldown: f32,
    pub hitbox_w: f32,
    pub hitbox_h: f32,
    pub hitbox_offset_x: f32,
    pub hitbox_offset_y: f32,
    pub active_frame_start: usize,
    pub active_frame_end: usize,
    pub knockback_force: f32,
    #[allow(dead_code)] // Will be rendered in weapon select UI
    pub description: &'static str,
    pub speed_label: &'static str,
    pub range_label: &'static str,
}

static SWORD: WeaponDef = WeaponDef {
    id: WeaponId::Sword,
    name: "Sword",
    base_damage: 2,
    attack_cooldown: 0.35,
    hitbox_w: 12.0,
    hitbox_h: 8.0,
    hitbox_offset_x: 8.0,
    hitbox_offset_y: 3.0,
    active_frame_start: 2,
    active_frame_end: 3,
    knockback_force: 60.0,
    description: "Reliable blade",
    speed_label: "Normal",
    range_label: "Medium",
};

static SPEAR: WeaponDef = WeaponDef {
    id: WeaponId::Spear,
    name: "Spear",
    base_damage: 3,
    attack_cooldown: 0.5,
    hitbox_w: 18.0,
    hitbox_h: 4.0,
    hitbox_offset_x: 10.0,
    hitbox_offset_y: 5.0,
    active_frame_start: 2,
    active_frame_end: 3,
    knockback_force: 80.0,
    description: "Precise thrust",
    speed_label: "Slow",
    range_label: "Long",
};

static DAGGERS: WeaponDef = WeaponDef {
    id: WeaponId::Daggers,
    name: "Daggers",
    base_damage: 1,
    attack_cooldown: 0.15,
    hitbox_w: 6.0,
    hitbox_h: 10.0,
    hitbox_offset_x: 5.0,
    hitbox_offset_y: 2.0,
    active_frame_start: 1,
    active_frame_end: 2,
    knockback_force: 30.0,
    description: "Rapid strikes",
    speed_label: "Fast",
    range_label: "Short",
};

pub fn get_weapon(id: WeaponId) -> &'static WeaponDef {
    match id {
        WeaponId::Sword => &SWORD,
        WeaponId::Spear => &SPEAR,
        WeaponId::Daggers => &DAGGERS,
    }
}

pub fn all_weapons() -> [&'static WeaponDef; 3] {
    [&SWORD, &SPEAR, &DAGGERS]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_weapon_returns_correct_sword() {
        let w = get_weapon(WeaponId::Sword);
        assert_eq!(w.id, WeaponId::Sword);
        assert_eq!(w.name, "Sword");
        assert_eq!(w.base_damage, 2);
        assert!((w.attack_cooldown - 0.35).abs() < f32::EPSILON);
        assert!((w.knockback_force - 60.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_get_weapon_returns_correct_spear() {
        let w = get_weapon(WeaponId::Spear);
        assert_eq!(w.id, WeaponId::Spear);
        assert_eq!(w.name, "Spear");
        assert_eq!(w.base_damage, 3);
        assert!((w.attack_cooldown - 0.5).abs() < f32::EPSILON);
        assert!((w.knockback_force - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_get_weapon_returns_correct_daggers() {
        let w = get_weapon(WeaponId::Daggers);
        assert_eq!(w.id, WeaponId::Daggers);
        assert_eq!(w.name, "Daggers");
        assert_eq!(w.base_damage, 1);
        assert!((w.attack_cooldown - 0.15).abs() < f32::EPSILON);
        assert!((w.knockback_force - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_all_weapons_returns_three_distinct() {
        let weapons = all_weapons();
        assert_eq!(weapons.len(), 3);
        assert_eq!(weapons[0].id, WeaponId::Sword);
        assert_eq!(weapons[1].id, WeaponId::Spear);
        assert_eq!(weapons[2].id, WeaponId::Daggers);
    }

    #[test]
    fn test_sword_is_balanced() {
        let sword = get_weapon(WeaponId::Sword);
        let spear = get_weapon(WeaponId::Spear);
        let daggers = get_weapon(WeaponId::Daggers);

        // Sword damage is between daggers and spear
        assert!(sword.base_damage > daggers.base_damage);
        assert!(sword.base_damage < spear.base_damage);

        // Sword cooldown is between daggers and spear
        assert!(sword.attack_cooldown > daggers.attack_cooldown);
        assert!(sword.attack_cooldown < spear.attack_cooldown);
    }

    #[test]
    fn test_spear_has_longest_range() {
        let weapons = all_weapons();
        let spear = get_weapon(WeaponId::Spear);
        for w in &weapons {
            assert!(spear.hitbox_w >= w.hitbox_w);
            assert!(spear.hitbox_offset_x >= w.hitbox_offset_x);
        }
    }

    #[test]
    fn test_daggers_fastest_cooldown() {
        let weapons = all_weapons();
        let daggers = get_weapon(WeaponId::Daggers);
        for w in &weapons {
            assert!(daggers.attack_cooldown <= w.attack_cooldown);
        }
    }

    #[test]
    fn test_weapon_labels_non_empty() {
        for w in &all_weapons() {
            assert!(!w.speed_label.is_empty());
            assert!(!w.range_label.is_empty());
            assert!(!w.description.is_empty());
            assert!(!w.name.is_empty());
        }
    }

    #[test]
    fn test_active_frames_valid() {
        for w in &all_weapons() {
            assert!(w.active_frame_start <= w.active_frame_end);
            assert!(w.active_frame_start > 0, "active frames should not start at frame 0 (wind-up needed)");
        }
    }

    #[test]
    fn test_hitbox_dimensions_positive() {
        for w in &all_weapons() {
            assert!(w.hitbox_w > 0.0);
            assert!(w.hitbox_h > 0.0);
            assert!(w.hitbox_offset_x > 0.0);
        }
    }
}
