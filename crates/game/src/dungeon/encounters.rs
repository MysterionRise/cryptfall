//! Encounter definitions for dungeon rooms.
//!
//! Each combat room selects an `EncounterDef` based on difficulty and floor number.
//! An encounter consists of one or more waves of enemies that spawn in sequence.

use crate::enemies::{Enemy, EnemyType};
use engine::tilemap::TILE_SIZE;

use super::room_template::SpawnPoint;

/// A complete encounter for a room.
pub struct EncounterDef {
    pub waves: Vec<WaveDef>,
}

/// A single wave within an encounter.
pub struct WaveDef {
    pub enemies: Vec<EnemySpawn>,
    pub trigger: WaveTrigger,
}

/// An enemy to spawn as part of a wave.
pub struct EnemySpawn {
    pub enemy_type: EnemyType,
    /// Index into the room's spawn_points array (wraps around if out of bounds).
    pub spawn_index: usize,
}

/// When a wave should activate.
pub enum WaveTrigger {
    /// Spawn when room combat starts.
    Immediate,
    /// Spawn after the previous wave is fully cleared.
    OnPreviousWaveCleared,
    /// Spawn when fewer than N enemies remain alive.
    OnEnemyCountBelow(usize),
}

/// Difficulty tier for encounter selection.
#[derive(Clone, Copy)]
pub enum EncounterDifficulty {
    Easy,
    Medium,
    Hard,
    Boss,
}

/// Select an encounter based on difficulty, floor, and available spawn points.
pub fn select_encounter(
    difficulty: EncounterDifficulty,
    floor_number: u32,
    num_spawn_points: usize,
    seed: u64,
) -> EncounterDef {
    if num_spawn_points == 0 {
        return EncounterDef { waves: Vec::new() };
    }

    // Floor scaling: each floor beyond 1 adds extra enemies
    let floor_bonus_sk = ((floor_number.saturating_sub(1)) as usize).min(3);
    let floor_bonus_gh = ((floor_number.saturating_sub(2)) as usize).min(2);

    // Use seed to add variety within the same difficulty tier
    let variant = (seed % 3) as usize;

    match difficulty {
        EncounterDifficulty::Easy => {
            // Single wave, skeletons only
            let sk_count = (2 + floor_bonus_sk + variant % 2).min(num_spawn_points);
            EncounterDef {
                waves: vec![WaveDef {
                    enemies: make_spawns(sk_count, 0, 0),
                    trigger: WaveTrigger::Immediate,
                }],
            }
        }
        EncounterDifficulty::Medium => {
            // Two waves: skeletons, then skeletons + ghosts
            let w1_sk = (2 + floor_bonus_sk).min(num_spawn_points);
            let w2_sk = (1 + floor_bonus_sk / 2).min(num_spawn_points);
            let w2_gh = (1 + floor_bonus_gh).min(num_spawn_points.saturating_sub(w2_sk));
            EncounterDef {
                waves: vec![
                    WaveDef {
                        enemies: make_spawns(w1_sk, 0, 0),
                        trigger: WaveTrigger::Immediate,
                    },
                    WaveDef {
                        enemies: make_spawns(w2_sk, w2_gh, w1_sk),
                        trigger: WaveTrigger::OnPreviousWaveCleared,
                    },
                ],
            }
        }
        EncounterDifficulty::Hard => {
            // Two waves, both with ghosts
            let w1_sk = (3 + floor_bonus_sk).min(num_spawn_points);
            let w1_gh = (1 + floor_bonus_gh).min(num_spawn_points.saturating_sub(w1_sk));
            let w2_sk = (2 + floor_bonus_sk).min(num_spawn_points);
            let w2_gh = (2 + floor_bonus_gh).min(num_spawn_points.saturating_sub(w2_sk));
            EncounterDef {
                waves: vec![
                    WaveDef {
                        enemies: make_spawns(w1_sk, w1_gh, 0),
                        trigger: WaveTrigger::Immediate,
                    },
                    WaveDef {
                        enemies: make_spawns(w2_sk, w2_gh, w1_sk + w1_gh),
                        trigger: WaveTrigger::OnEnemyCountBelow(2),
                    },
                ],
            }
        }
        EncounterDifficulty::Boss => {
            // Single wave: one Bone King boss
            EncounterDef {
                waves: vec![WaveDef {
                    enemies: vec![EnemySpawn {
                        enemy_type: EnemyType::BoneKing,
                        spawn_index: 0,
                    }],
                    trigger: WaveTrigger::Immediate,
                }],
            }
        }
    }
}

/// Build a list of `EnemySpawn`s: skeletons first, then ghosts.
/// `offset` shifts spawn_index so later waves use different spawn points.
fn make_spawns(skeleton_count: usize, ghost_count: usize, offset: usize) -> Vec<EnemySpawn> {
    let total = skeleton_count + ghost_count;
    let mut spawns = Vec::with_capacity(total);
    for i in 0..skeleton_count {
        spawns.push(EnemySpawn {
            enemy_type: EnemyType::Skeleton,
            spawn_index: offset + i,
        });
    }
    for i in 0..ghost_count {
        spawns.push(EnemySpawn {
            enemy_type: EnemyType::Ghost,
            spawn_index: offset + skeleton_count + i,
        });
    }
    spawns
}

/// Instantiate `Enemy` objects from a wave's spawn list using room spawn points.
pub fn instantiate_wave(
    wave: &WaveDef,
    spawn_points: &[SpawnPoint],
    room_index: usize,
    base_seed: u64,
) -> Vec<Enemy> {
    if spawn_points.is_empty() {
        return Vec::new();
    }

    let mut enemies = Vec::with_capacity(wave.enemies.len());
    for (i, es) in wave.enemies.iter().enumerate() {
        let sp = &spawn_points[es.spawn_index % spawn_points.len()];
        let px = (sp.x * TILE_SIZE) as f32;
        let py = (sp.y * TILE_SIZE) as f32;
        let seed = (room_index as u32)
            .wrapping_mul(31337)
            .wrapping_add(base_seed as u32)
            .wrapping_add(i as u32 * 7919 + 1);

        let enemy = match es.enemy_type {
            EnemyType::Skeleton => Enemy::new_skeleton(px, py, seed),
            EnemyType::Ghost => Enemy::new_ghost(px, py, seed),
            EnemyType::Slime => Enemy::new_slime(px, py),
            EnemyType::BoneKing => Enemy::new_bone_king(px, py, seed),
        };
        enemies.push(enemy);
    }
    enemies
}

/// Tracks wave progression during combat in a room.
pub struct WaveTracker {
    pub encounter: EncounterDef,
    pub current_wave: usize,
    /// Number of enemies spawned by completed waves (used to detect wave clears)
    pub wave_spawned_count: usize,
}

impl WaveTracker {
    pub fn new(encounter: EncounterDef) -> Self {
        Self {
            current_wave: 0,
            wave_spawned_count: 0,
            encounter,
        }
    }

    /// Check if there are more waves to spawn.
    pub fn has_more_waves(&self) -> bool {
        self.current_wave < self.encounter.waves.len()
    }

    /// Check if the current wave's trigger condition is met.
    /// `alive_count` = number of currently alive enemies.
    pub fn should_spawn_next_wave(&self, alive_count: usize) -> bool {
        if self.current_wave >= self.encounter.waves.len() {
            return false;
        }

        let wave = &self.encounter.waves[self.current_wave];
        match wave.trigger {
            WaveTrigger::Immediate => true,
            WaveTrigger::OnPreviousWaveCleared => alive_count == 0,
            WaveTrigger::OnEnemyCountBelow(n) => alive_count < n,
        }
    }

    /// Advance to the next wave, returning a reference to the wave to spawn.
    /// Returns None if no more waves.
    pub fn advance(&mut self) -> Option<&WaveDef> {
        if self.current_wave >= self.encounter.waves.len() {
            return None;
        }
        let wave = &self.encounter.waves[self.current_wave];
        self.wave_spawned_count += wave.enemies.len();
        self.current_wave += 1;
        Some(wave)
    }

    /// Check if the entire encounter is complete (all waves spawned, all enemies dead).
    pub fn is_encounter_complete(&self, alive_count: usize) -> bool {
        !self.has_more_waves() && alive_count == 0 && self.wave_spawned_count > 0
    }
}
