use super::BoonId;

pub struct PlayerBoons {
    pub active: Vec<BoonId>,
    // Cached stat modifiers
    pub damage_flat_bonus: i32,
    pub damage_mult: f32,
    pub attack_speed_mult: f32,
    pub max_hp_bonus: i32,
    pub move_speed_mult: f32,
    pub dash_distance_mult: f32,
    pub dash_cooldown_reduction: f32,
    // Triggered effect flags
    pub on_hit_heal_chance: f32,
    pub on_kill_explode_damage: i32,
    pub on_dash_damage: i32,
    pub shield_charges: i32,
    pub shield_max: i32,
    pub has_projectile_attack: bool,
    pub chain_lightning_targets: i32,
    pub life_steal_percent: f32,
    pub has_second_wind: bool,
    pub second_wind_used: bool,
    pub gold_mult: f32,
    pub lucky: bool,
    pub treasure_sense: bool,
    pub crit_chance: f32,
    pub fury_kills_this_room: i32,
    pub has_retaliation: bool,
    pub has_deaths_bargain: bool,
}

impl PlayerBoons {
    pub fn new() -> Self {
        Self {
            active: Vec::new(),
            damage_flat_bonus: 0,
            damage_mult: 1.0,
            attack_speed_mult: 1.0,
            max_hp_bonus: 0,
            move_speed_mult: 1.0,
            dash_distance_mult: 1.0,
            dash_cooldown_reduction: 0.0,
            on_hit_heal_chance: 0.0,
            on_kill_explode_damage: 0,
            on_dash_damage: 0,
            shield_charges: 0,
            shield_max: 0,
            has_projectile_attack: false,
            chain_lightning_targets: 0,
            life_steal_percent: 0.0,
            has_second_wind: false,
            second_wind_used: false,
            gold_mult: 1.0,
            lucky: false,
            treasure_sense: false,
            crit_chance: 0.0,
            fury_kills_this_room: 0,
            has_retaliation: false,
            has_deaths_bargain: false,
        }
    }

    pub fn add(&mut self, boon_id: BoonId) {
        self.active.push(boon_id);
        self.recalculate();
    }

    pub fn recalculate(&mut self) {
        // Reset all cached values to defaults
        self.damage_flat_bonus = 0;
        self.damage_mult = 1.0;
        self.attack_speed_mult = 1.0;
        self.max_hp_bonus = 0;
        self.move_speed_mult = 1.0;
        self.dash_distance_mult = 1.0;
        self.dash_cooldown_reduction = 0.0;
        self.on_hit_heal_chance = 0.0;
        self.on_kill_explode_damage = 0;
        self.on_dash_damage = 0;
        self.shield_max = 0;
        self.has_projectile_attack = false;
        self.chain_lightning_targets = 0;
        self.life_steal_percent = 0.0;
        self.has_second_wind = false;
        self.gold_mult = 1.0;
        self.lucky = false;
        self.treasure_sense = false;
        self.crit_chance = 0.0;
        self.has_retaliation = false;
        self.has_deaths_bargain = false;

        for &boon in &self.active {
            match boon {
                BoonId::SharpenedBlade => {
                    self.damage_flat_bonus += 1;
                }
                BoonId::BerserkersRage => {
                    self.damage_mult += 0.25;
                }
                BoonId::SwiftStrikes => {
                    self.attack_speed_mult *= 0.7;
                }
                BoonId::KillingBlow => {
                    self.on_kill_explode_damage = 2;
                }
                BoonId::ChainLightning => {
                    self.chain_lightning_targets = 2;
                }
                BoonId::ProjectileSlash => {
                    self.has_projectile_attack = true;
                }
                BoonId::CriticalEdge => {
                    self.crit_chance = 0.2;
                }
                BoonId::Fury => {
                    // Damage bonus applied dynamically via fury_damage_mult()
                }
                BoonId::ToughSkin => {
                    self.max_hp_bonus += 1;
                }
                BoonId::IronShield => {
                    self.shield_max = 2;
                    // shield_charges are set by reset_floor_state, not recalculate
                }
                BoonId::LifeSteal => {
                    self.life_steal_percent = 0.15;
                }
                BoonId::VampiricTouch => {
                    self.on_hit_heal_chance = 0.1;
                }
                BoonId::Retaliation => {
                    self.has_retaliation = true;
                }
                BoonId::SecondWind => {
                    self.has_second_wind = true;
                }
                BoonId::SwiftFeet => {
                    self.move_speed_mult += 0.2;
                }
                BoonId::PhantomDash => {
                    self.dash_distance_mult += 0.4;
                }
                BoonId::ShadowStep => {
                    self.dash_cooldown_reduction += 0.5;
                }
                BoonId::DashStrike => {
                    self.on_dash_damage = 2;
                }
                BoonId::GoldMagnet => {
                    self.gold_mult += 0.5;
                }
                BoonId::Lucky => {
                    self.lucky = true;
                }
                BoonId::TreasureSense => {
                    self.treasure_sense = true;
                }
                BoonId::DeathsBargain => {
                    self.damage_flat_bonus += 3;
                    self.has_deaths_bargain = true;
                    // max_hp_bonus is set to reduce max HP to 1
                    // The game integration layer will handle this based on player's base max HP
                }
            }
        }
    }

    /// Returns the Fury damage multiplier based on kills this room.
    /// Each kill grants +5% damage.
    #[allow(dead_code)] // Used by tests; will be called from combat damage calc in Phase 5
    pub fn fury_damage_mult(&self) -> f32 {
        if self.has_boon(BoonId::Fury) {
            1.0 + self.fury_kills_this_room as f32 * 0.05
        } else {
            1.0
        }
    }

    /// Calculate effective max HP bonus, accounting for Death's Bargain.
    /// `base_max_hp` is the player's max HP before any boon modifiers.
    pub fn effective_max_hp_bonus(&self, base_max_hp: i32) -> i32 {
        if self.has_deaths_bargain {
            // Death's Bargain overrides everything: final max HP = 1
            1 - base_max_hp
        } else {
            self.max_hp_bonus
        }
    }

    #[allow(dead_code)] // Used by fury_damage_mult and tests; general-purpose query method
    pub fn has_boon(&self, id: BoonId) -> bool {
        self.active.contains(&id)
    }

    /// Reset per-room state (fury kill counter).
    pub fn reset_room_state(&mut self) {
        self.fury_kills_this_room = 0;
    }

    /// Reset per-floor state (shield charges, second wind).
    pub fn reset_floor_state(&mut self) {
        self.shield_charges = self.shield_max;
        self.second_wind_used = false;
    }

    /// Record a kill for Fury tracking.
    pub fn record_kill(&mut self) {
        self.fury_kills_this_room += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let boons = PlayerBoons::new();
        assert_eq!(boons.damage_flat_bonus, 0);
        assert_eq!(boons.damage_mult, 1.0);
        assert_eq!(boons.attack_speed_mult, 1.0);
        assert_eq!(boons.max_hp_bonus, 0);
        assert_eq!(boons.move_speed_mult, 1.0);
        assert!(boons.active.is_empty());
    }

    #[test]
    fn test_sharpened_blade_stacks() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::SharpenedBlade);
        assert_eq!(boons.damage_flat_bonus, 1);
        boons.add(BoonId::SharpenedBlade);
        assert_eq!(boons.damage_flat_bonus, 2);
        boons.add(BoonId::SharpenedBlade);
        assert_eq!(boons.damage_flat_bonus, 3);
    }

    #[test]
    fn test_tough_skin_stacks() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::ToughSkin);
        boons.add(BoonId::ToughSkin);
        assert_eq!(boons.max_hp_bonus, 2);
    }

    #[test]
    fn test_swift_feet_stacks() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::SwiftFeet);
        assert!((boons.move_speed_mult - 1.2).abs() < 0.001);
        boons.add(BoonId::SwiftFeet);
        assert!((boons.move_speed_mult - 1.4).abs() < 0.001);
    }

    #[test]
    fn test_berserkers_rage() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::BerserkersRage);
        assert!((boons.damage_mult - 1.25).abs() < 0.001);
    }

    #[test]
    fn test_swift_strikes() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::SwiftStrikes);
        assert!((boons.attack_speed_mult - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_iron_shield_floor_reset() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::IronShield);
        assert_eq!(boons.shield_max, 2);
        assert_eq!(boons.shield_charges, 0); // Not set until reset_floor_state
        boons.reset_floor_state();
        assert_eq!(boons.shield_charges, 2);
        boons.shield_charges -= 1;
        assert_eq!(boons.shield_charges, 1);
        boons.reset_floor_state();
        assert_eq!(boons.shield_charges, 2);
    }

    #[test]
    fn test_second_wind_floor_reset() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::SecondWind);
        assert!(boons.has_second_wind);
        assert!(!boons.second_wind_used);
        boons.second_wind_used = true;
        boons.reset_floor_state();
        assert!(!boons.second_wind_used);
    }

    #[test]
    fn test_fury_damage() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::Fury);
        assert!((boons.fury_damage_mult() - 1.0).abs() < 0.001);
        boons.record_kill();
        assert!((boons.fury_damage_mult() - 1.05).abs() < 0.001);
        boons.record_kill();
        boons.record_kill();
        assert!((boons.fury_damage_mult() - 1.15).abs() < 0.001);
        boons.reset_room_state();
        assert!((boons.fury_damage_mult() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_deaths_bargain() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::DeathsBargain);
        assert_eq!(boons.damage_flat_bonus, 3);
        assert!(boons.has_deaths_bargain);
        // With base max HP of 5, effective bonus should bring it to 1
        let bonus = boons.effective_max_hp_bonus(5);
        assert_eq!(5 + bonus, 1);
    }

    #[test]
    fn test_deaths_bargain_with_tough_skin() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::ToughSkin);
        boons.add(BoonId::ToughSkin);
        boons.add(BoonId::DeathsBargain);
        assert_eq!(boons.damage_flat_bonus, 3);
        // ToughSkin gives +2 max HP, but Death's Bargain overrides to 1
        let bonus = boons.effective_max_hp_bonus(5);
        assert_eq!(5 + bonus, 1);
    }

    #[test]
    fn test_combined_offense() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::SharpenedBlade);
        boons.add(BoonId::BerserkersRage);
        boons.add(BoonId::CriticalEdge);
        assert_eq!(boons.damage_flat_bonus, 1);
        assert!((boons.damage_mult - 1.25).abs() < 0.001);
        assert!((boons.crit_chance - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_mobility_boons() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::PhantomDash);
        boons.add(BoonId::ShadowStep);
        boons.add(BoonId::DashStrike);
        assert!((boons.dash_distance_mult - 1.4).abs() < 0.001);
        assert!((boons.dash_cooldown_reduction - 0.5).abs() < 0.001);
        assert_eq!(boons.on_dash_damage, 2);
    }

    #[test]
    fn test_special_boons() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::GoldMagnet);
        boons.add(BoonId::Lucky);
        boons.add(BoonId::TreasureSense);
        assert!((boons.gold_mult - 1.5).abs() < 0.001);
        assert!(boons.lucky);
        assert!(boons.treasure_sense);
    }

    #[test]
    fn test_has_boon() {
        let mut boons = PlayerBoons::new();
        assert!(!boons.has_boon(BoonId::Lucky));
        boons.add(BoonId::Lucky);
        assert!(boons.has_boon(BoonId::Lucky));
        assert!(!boons.has_boon(BoonId::Fury));
    }

    #[test]
    fn test_recalculate_resets_properly() {
        let mut boons = PlayerBoons::new();
        boons.add(BoonId::SharpenedBlade);
        boons.add(BoonId::BerserkersRage);
        assert_eq!(boons.damage_flat_bonus, 1);
        assert!((boons.damage_mult - 1.25).abs() < 0.001);

        // Remove BerserkersRage manually and recalculate
        boons.active.retain(|&b| b != BoonId::BerserkersRage);
        boons.recalculate();
        assert_eq!(boons.damage_flat_bonus, 1);
        assert!((boons.damage_mult - 1.0).abs() < 0.001);
    }
}
