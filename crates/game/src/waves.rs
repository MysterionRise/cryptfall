//! Wave management: spawning, progression, and victory tracking.

use crate::enemies::Enemy;

/// Maximum number of waves in the arena
pub const MAX_WAVES: u8 = 3;

/// Delay between clearing a wave and spawning the next (seconds)
pub const WAVE_TRANSITION_DELAY: f32 = 1.5;

/// Duration of the "WAVE N" announcement text (seconds)
pub const WAVE_ANNOUNCE_DURATION: f32 = 2.0;

/// Tracks wave state: current wave, transition timing, and victory.
pub struct WaveManager {
    pub current_wave: u8,
    pub wave_clear_timer: f32,
    pub wave_announce_timer: f32,
    pub wave_clear_display_timer: f32,
    pub victory: bool,
}

impl WaveManager {
    pub fn new() -> Self {
        Self {
            current_wave: 1,
            wave_clear_timer: 0.0,
            wave_announce_timer: WAVE_ANNOUNCE_DURATION,
            wave_clear_display_timer: 0.0,
            victory: false,
        }
    }

    /// Tick the announce timer.
    pub fn update_announce(&mut self, dt: f32) {
        if self.wave_announce_timer > 0.0 {
            self.wave_announce_timer -= dt;
        }
    }

    /// Tick the wave clear display timer.
    pub fn update_clear_display(&mut self, dt: f32) {
        if self.wave_clear_display_timer > 0.0 {
            self.wave_clear_display_timer -= dt;
        }
    }

    /// Check wave progression. Returns `Some(new_enemies)` if a new wave should spawn.
    pub fn check_progression(&mut self, enemies: &[Enemy], dt: f32) -> Option<Vec<Enemy>> {
        if self.victory {
            return None;
        }

        let all_dead = enemies
            .iter()
            .all(|e| !e.alive && e.animation.is_finished());

        if all_dead && !enemies.is_empty() {
            // Set wave clear display timer once per wave clear
            if self.wave_clear_display_timer == 0.0 {
                self.wave_clear_display_timer = 1.5;
            }
            self.wave_clear_timer += dt;
            if self.wave_clear_timer >= WAVE_TRANSITION_DELAY {
                if self.current_wave < MAX_WAVES {
                    self.current_wave += 1;
                    self.wave_clear_timer = 0.0;
                    self.wave_clear_display_timer = 0.0;
                    self.wave_announce_timer = WAVE_ANNOUNCE_DURATION;
                    return Some(spawn_wave(self.current_wave));
                } else {
                    self.victory = true;
                }
            }
        } else {
            self.wave_clear_timer = 0.0;
        }

        None
    }

    /// Spawn the initial wave's enemies.
    pub fn spawn_initial(&self) -> Vec<Enemy> {
        spawn_wave(self.current_wave)
    }

    /// Reset to wave 1 for restart.
    pub fn reset(&mut self) {
        self.current_wave = 1;
        self.wave_clear_timer = 0.0;
        self.wave_announce_timer = WAVE_ANNOUNCE_DURATION;
        self.wave_clear_display_timer = 0.0;
        self.victory = false;
    }
}

fn spawn_wave(wave: u8) -> Vec<Enemy> {
    match wave {
        1 => vec![
            // Wave 1: 3 skeletons
            Enemy::new_skeleton(24.0, 24.0, 11111),
            Enemy::new_skeleton(128.0, 24.0, 22222),
            Enemy::new_skeleton(24.0, 88.0, 33333),
        ],
        2 => vec![
            // Wave 2: 2 skeletons + 1 ghost
            Enemy::new_skeleton(24.0, 24.0, 44444),
            Enemy::new_skeleton(128.0, 88.0, 55555),
            Enemy::new_ghost(128.0, 24.0, 66666),
        ],
        3 => vec![
            // Wave 3: 4 skeletons + 2 ghosts
            Enemy::new_skeleton(24.0, 24.0, 77777),
            Enemy::new_skeleton(128.0, 24.0, 88888),
            Enemy::new_skeleton(24.0, 88.0, 99999),
            Enemy::new_skeleton(128.0, 88.0, 11222),
            Enemy::new_ghost(76.0, 16.0, 33444),
            Enemy::new_ghost(76.0, 96.0, 55666),
        ],
        _ => vec![],
    }
}
