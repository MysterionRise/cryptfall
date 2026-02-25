/// Gold reward values for each enemy type.
pub const GOLD_SKELETON: u32 = 2;
pub const GOLD_GHOST: u32 = 3;
pub const GOLD_BONE_KING: u32 = 25;
pub const GOLD_ROOM_CLEAR_BONUS: u32 = 5;

/// Tracks per-run statistics for end-of-run summary and save integration.
pub struct RunState {
    pub kills: u32,
    #[allow(dead_code)] // Tracked for run-end summary screen
    pub damage_dealt: u32,
    #[allow(dead_code)] // Tracked for run-end summary screen
    pub damage_taken: u32,
    pub gold_earned: u32,
    pub boons_collected: u32,
    pub rooms_cleared: u32,
    pub floor_reached: u32,
    pub elapsed_secs: f32,
}

impl RunState {
    pub fn new() -> Self {
        Self {
            kills: 0,
            damage_dealt: 0,
            damage_taken: 0,
            gold_earned: 0,
            boons_collected: 0,
            rooms_cleared: 0,
            floor_reached: 0,
            elapsed_secs: 0.0,
        }
    }

    pub fn record_kill(&mut self, enemy_gold: u32) {
        self.kills += 1;
        self.gold_earned += enemy_gold;
    }

    #[allow(dead_code)] // Will be called from combat system for run-end stats
    pub fn record_damage_dealt(&mut self, amount: u32) {
        self.damage_dealt += amount;
    }

    #[allow(dead_code)] // Will be called from combat system for run-end stats
    pub fn record_damage_taken(&mut self, amount: u32) {
        self.damage_taken += amount;
    }

    pub fn record_room_clear(&mut self, bonus_gold: u32) {
        self.rooms_cleared += 1;
        self.gold_earned += bonus_gold;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_run_state_zeroed() {
        let rs = RunState::new();
        assert_eq!(rs.kills, 0);
        assert_eq!(rs.damage_dealt, 0);
        assert_eq!(rs.damage_taken, 0);
        assert_eq!(rs.gold_earned, 0);
        assert_eq!(rs.boons_collected, 0);
        assert_eq!(rs.rooms_cleared, 0);
        assert_eq!(rs.floor_reached, 0);
        assert!((rs.elapsed_secs - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_record_kill_increments_kills_and_gold() {
        let mut rs = RunState::new();
        rs.record_kill(GOLD_SKELETON);
        assert_eq!(rs.kills, 1);
        assert_eq!(rs.gold_earned, 2);

        rs.record_kill(GOLD_GHOST);
        assert_eq!(rs.kills, 2);
        assert_eq!(rs.gold_earned, 5);

        rs.record_kill(GOLD_BONE_KING);
        assert_eq!(rs.kills, 3);
        assert_eq!(rs.gold_earned, 30);
    }

    #[test]
    fn test_record_damage_dealt() {
        let mut rs = RunState::new();
        rs.record_damage_dealt(5);
        rs.record_damage_dealt(3);
        assert_eq!(rs.damage_dealt, 8);
    }

    #[test]
    fn test_record_damage_taken() {
        let mut rs = RunState::new();
        rs.record_damage_taken(1);
        rs.record_damage_taken(2);
        assert_eq!(rs.damage_taken, 3);
    }

    #[test]
    fn test_record_room_clear() {
        let mut rs = RunState::new();
        rs.record_room_clear(GOLD_ROOM_CLEAR_BONUS);
        assert_eq!(rs.rooms_cleared, 1);
        assert_eq!(rs.gold_earned, 5);

        rs.record_room_clear(GOLD_ROOM_CLEAR_BONUS);
        assert_eq!(rs.rooms_cleared, 2);
        assert_eq!(rs.gold_earned, 10);
    }

    #[test]
    fn test_gold_constants() {
        assert_eq!(GOLD_SKELETON, 2);
        assert_eq!(GOLD_GHOST, 3);
        assert_eq!(GOLD_BONE_KING, 25);
        assert_eq!(GOLD_ROOM_CLEAR_BONUS, 5);
    }

    #[test]
    fn test_mixed_operations() {
        let mut rs = RunState::new();
        rs.record_kill(GOLD_SKELETON);
        rs.record_kill(GOLD_SKELETON);
        rs.record_kill(GOLD_GHOST);
        rs.record_room_clear(GOLD_ROOM_CLEAR_BONUS);
        rs.record_damage_dealt(10);
        rs.record_damage_taken(3);

        assert_eq!(rs.kills, 3);
        assert_eq!(rs.gold_earned, 12); // 2+2+3+5
        assert_eq!(rs.damage_dealt, 10);
        assert_eq!(rs.damage_taken, 3);
        assert_eq!(rs.rooms_cleared, 1);
    }
}
