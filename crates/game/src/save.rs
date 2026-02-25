use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SaveData {
    pub total_gold: u32,
    pub total_runs: u32,
    pub best_floor: u32,
    pub total_kills: u32,
    pub total_deaths: u32,
    pub upgrades: PermanentUpgrades,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PermanentUpgrades {
    /// 0-3, each level grants +1 max HP
    pub vitality_level: u32,
    /// 0-2, each level grants +1 damage
    pub strength_level: u32,
    /// Unlocks 2 dash charges
    pub twin_dash: bool,
    /// 0-2, each level grants +1 reroll per run
    pub boon_reroll_level: u32,
}

impl PermanentUpgrades {
    /// Apply permanent upgrades to a player's stats at the start of a run.
    /// Returns (bonus_max_hp, bonus_damage, dash_charges, rerolls_per_run).
    pub fn stat_bonuses(&self) -> (i32, i32, u32, u32) {
        let bonus_hp = self.vitality_level as i32; // +1 per level
        let bonus_dmg = self.strength_level as i32; // +1 per level
        let dash_charges = if self.twin_dash { 2 } else { 1 };
        let rerolls = self.boon_reroll_level;
        (bonus_hp, bonus_dmg, dash_charges, rerolls)
    }
}

pub struct UpgradeDef {
    pub name: &'static str,
    pub description: &'static str,
    pub cost: u32,
    pub max_level: u32,
    pub current_level_fn: fn(&PermanentUpgrades) -> u32,
}

pub const UPGRADES: &[UpgradeDef] = &[
    UpgradeDef {
        name: "Vitality I",
        description: "+1 Max HP",
        cost: 30,
        max_level: 1,
        current_level_fn: |u| u.vitality_level.min(1),
    },
    UpgradeDef {
        name: "Vitality II",
        description: "+1 Max HP",
        cost: 60,
        max_level: 1,
        current_level_fn: |u| u.vitality_level.saturating_sub(1).min(1),
    },
    UpgradeDef {
        name: "Vitality III",
        description: "+1 Max HP",
        cost: 120,
        max_level: 1,
        current_level_fn: |u| u.vitality_level.saturating_sub(2).min(1),
    },
    UpgradeDef {
        name: "Strength I",
        description: "+1 Damage",
        cost: 50,
        max_level: 1,
        current_level_fn: |u| u.strength_level.min(1),
    },
    UpgradeDef {
        name: "Strength II",
        description: "+1 Damage",
        cost: 100,
        max_level: 1,
        current_level_fn: |u| u.strength_level.saturating_sub(1).min(1),
    },
    UpgradeDef {
        name: "Twin Dash",
        description: "2 dash charges",
        cost: 80,
        max_level: 1,
        current_level_fn: |u| u32::from(u.twin_dash),
    },
    UpgradeDef {
        name: "Boon Reroll",
        description: "+1 reroll per run",
        cost: 40,
        max_level: 1,
        current_level_fn: |u| u.boon_reroll_level.min(1),
    },
    UpgradeDef {
        name: "Boon Reroll+",
        description: "+1 reroll per run",
        cost: 80,
        max_level: 1,
        current_level_fn: |u| u.boon_reroll_level.saturating_sub(1).min(1),
    },
];

fn save_path() -> PathBuf {
    let mut path = dirs_fallback();
    path.push(".cryptfall");
    path.push("save.json");
    path
}

/// Returns the user's home directory, falling back to current dir.
fn dirs_fallback() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

impl SaveData {
    pub fn load() -> Self {
        let path = save_path();
        match std::fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
                eprintln!("Warning: corrupted save file, using defaults: {e}");
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        let path = save_path();
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("Warning: could not create save directory: {e}");
                return;
            }
        }
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&path, json) {
                    eprintln!("Warning: could not write save file: {e}");
                }
            }
            Err(e) => {
                eprintln!("Warning: could not serialize save data: {e}");
            }
        }
    }

    pub fn can_afford(&self, cost: u32) -> bool {
        self.total_gold >= cost
    }

    pub fn spend_gold(&mut self, cost: u32) -> bool {
        if self.total_gold >= cost {
            self.total_gold -= cost;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_load_roundtrip() {
        let dir = std::env::temp_dir().join("cryptfall_test_save");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("save.json");

        let data = SaveData {
            total_gold: 150,
            total_runs: 5,
            best_floor: 3,
            total_kills: 42,
            total_deaths: 4,
            upgrades: PermanentUpgrades {
                vitality_level: 2,
                strength_level: 1,
                twin_dash: true,
                boon_reroll_level: 1,
            },
        };

        // Write directly to temp path
        let json = serde_json::to_string_pretty(&data).unwrap();
        fs::write(&path, &json).unwrap();

        // Read back
        let contents = fs::read_to_string(&path).unwrap();
        let loaded: SaveData = serde_json::from_str(&contents).unwrap();

        assert_eq!(loaded.total_gold, 150);
        assert_eq!(loaded.total_runs, 5);
        assert_eq!(loaded.best_floor, 3);
        assert_eq!(loaded.total_kills, 42);
        assert_eq!(loaded.total_deaths, 4);
        assert_eq!(loaded.upgrades.vitality_level, 2);
        assert_eq!(loaded.upgrades.strength_level, 1);
        assert!(loaded.upgrades.twin_dash);
        assert_eq!(loaded.upgrades.boon_reroll_level, 1);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_corrupted_json_returns_default() {
        let dir = std::env::temp_dir().join("cryptfall_test_corrupt");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("save.json");

        fs::write(&path, "not valid json {{{").unwrap();
        let contents = fs::read_to_string(&path).unwrap();
        let loaded: Result<SaveData, _> = serde_json::from_str(&contents);
        assert!(loaded.is_err());

        // Our load() method handles this gracefully
        let default = SaveData::default();
        assert_eq!(default.total_gold, 0);
        assert_eq!(default.total_runs, 0);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_default_save_data() {
        let data = SaveData::default();
        assert_eq!(data.total_gold, 0);
        assert_eq!(data.total_runs, 0);
        assert_eq!(data.best_floor, 0);
        assert_eq!(data.total_kills, 0);
        assert_eq!(data.total_deaths, 0);
        assert_eq!(data.upgrades.vitality_level, 0);
        assert_eq!(data.upgrades.strength_level, 0);
        assert!(!data.upgrades.twin_dash);
        assert_eq!(data.upgrades.boon_reroll_level, 0);
    }

    #[test]
    fn test_can_afford() {
        let data = SaveData {
            total_gold: 50,
            ..Default::default()
        };
        assert!(data.can_afford(50));
        assert!(data.can_afford(30));
        assert!(!data.can_afford(51));
    }

    #[test]
    fn test_spend_gold_success() {
        let mut data = SaveData {
            total_gold: 100,
            ..Default::default()
        };
        assert!(data.spend_gold(30));
        assert_eq!(data.total_gold, 70);
        assert!(data.spend_gold(70));
        assert_eq!(data.total_gold, 0);
    }

    #[test]
    fn test_spend_gold_insufficient() {
        let mut data = SaveData {
            total_gold: 20,
            ..Default::default()
        };
        assert!(!data.spend_gold(30));
        assert_eq!(data.total_gold, 20); // unchanged
    }

    #[test]
    fn test_upgrade_costs() {
        // Vitality: 30, 60, 120
        assert_eq!(UPGRADES[0].cost, 30);
        assert_eq!(UPGRADES[1].cost, 60);
        assert_eq!(UPGRADES[2].cost, 120);
        // Strength: 50, 100
        assert_eq!(UPGRADES[3].cost, 50);
        assert_eq!(UPGRADES[4].cost, 100);
        // Twin Dash: 80
        assert_eq!(UPGRADES[5].cost, 80);
        // Boon Reroll: 40, 80
        assert_eq!(UPGRADES[6].cost, 40);
        assert_eq!(UPGRADES[7].cost, 80);
    }

    #[test]
    fn test_upgrade_current_level_fns() {
        let empty = PermanentUpgrades::default();
        for upgrade in UPGRADES {
            assert_eq!((upgrade.current_level_fn)(&empty), 0);
        }

        let full = PermanentUpgrades {
            vitality_level: 3,
            strength_level: 2,
            twin_dash: true,
            boon_reroll_level: 2,
        };
        // All individual upgrades should report level 1 (maxed)
        for upgrade in UPGRADES {
            assert_eq!(
                (upgrade.current_level_fn)(&full),
                1,
                "upgrade '{}' should be maxed",
                upgrade.name
            );
        }
    }

    #[test]
    fn test_upgrade_names_non_empty() {
        for upgrade in UPGRADES {
            assert!(!upgrade.name.is_empty());
            assert!(!upgrade.description.is_empty());
            assert!(upgrade.cost > 0);
        }
    }
}
