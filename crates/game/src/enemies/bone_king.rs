use engine::collision::AABB;

// --- Boss constants ---
const BOSS_HP: i32 = 20;
const BOSS_SPEED: f32 = 30.0;
const BOSS_SPEED_PHASE2: f32 = 39.0; // +30%
const CHASE_RANGE: f32 = 60.0;
const ATTACK_RANGE: f32 = 35.0;

// --- Phase timing ---
const IDLE_DURATION: f32 = 0.5;
const SLAM_WINDUP: f32 = 0.6;
const SLAM_ACTIVE: f32 = 0.15;
const SLAM_RECOVER: f32 = 0.4;
const SWEEP_WINDUP: f32 = 0.5;
const SWEEP_ACTIVE: f32 = 0.2;
const SWEEP_RECOVER: f32 = 0.3;

// Phase 2 reduced cooldowns
const IDLE_DURATION_P2: f32 = 0.35;
const SLAM_WINDUP_P2: f32 = 0.45;
const SWEEP_WINDUP_P2: f32 = 0.35;

// Charge timing
const CHARGE_WINDUP: f32 = 0.4;
const CHARGE_SPEED: f32 = 100.0;
const CHARGE_MAX_DURATION: f32 = 1.5; // safety cap
const CHARGE_STUN: f32 = 1.0;

// Roar
const ROAR_DURATION: f32 = 0.8;

// Death
const DYING_DURATION: f32 = 1.5;

// Phase 2 HP threshold
const PHASE2_HP_THRESHOLD: i32 = 10;

// --- Attack hitboxes (local-space, relative to boss sprite origin) ---
// Boss sprite is 20x24. Center is around (10, 12).
// Slam: rectangle in front, 15x12
const SLAM_HITBOX_RIGHT: AABB = AABB::new(14.0, 8.0, 15.0, 12.0);
const SLAM_HITBOX_LEFT: AABB = AABB::new(-9.0, 8.0, 15.0, 12.0);
// Sweep: wider arc, 30x10
const SWEEP_HITBOX_RIGHT: AABB = AABB::new(8.0, 10.0, 30.0, 10.0);
const SWEEP_HITBOX_LEFT: AABB = AABB::new(-18.0, 10.0, 30.0, 10.0);
// Charge: body hitbox (full boss width)
const CHARGE_HITBOX: AABB = AABB::new(2.0, 4.0, 16.0, 16.0);

#[derive(Clone, Copy, PartialEq)]
pub enum BoneKingState {
    Idle,
    Chase,
    SlamWindup,
    SlamActive,
    SlamRecover,
    SweepWindup,
    SweepActive,
    SweepRecover,
    ChargeWindup,
    ChargeActive,
    ChargeStunned,
    Roar,
    Dying,
}

pub struct BoneKingAI {
    pub state: BoneKingState,
    timer: f32,
    /// Direction locked for attacks / charge
    attack_dir_x: f32,
    attack_dir_y: f32,
    /// Charge velocity
    charge_vx: f32,
    charge_vy: f32,
    /// Phase 2 activated
    pub phase2: bool,
    /// Phase 2 transition triggered (one-shot)
    phase2_triggered: bool,
    /// Alternates between slam and sweep
    next_is_slam: bool,
    /// Simple RNG state
    rng_state: u32,
    /// Track slam impact frame (one-shot per slam)
    slam_impact_fired: bool,
    /// Track charge wall hit (one-shot per charge)
    charge_wall_hit_fired: bool,
    /// Dying flash counter for visual effect
    pub dying_flash_counter: f32,
}

impl BoneKingAI {
    pub fn new(seed: u32) -> Self {
        Self {
            state: BoneKingState::Idle,
            timer: IDLE_DURATION,
            attack_dir_x: 1.0,
            attack_dir_y: 0.0,
            charge_vx: 0.0,
            charge_vy: 0.0,
            phase2: false,
            phase2_triggered: false,
            next_is_slam: true,
            rng_state: seed,
            slam_impact_fired: false,
            charge_wall_hit_fired: false,
            dying_flash_counter: 0.0,
        }
    }

    fn rand_float(&mut self) -> f32 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(1103515245)
            .wrapping_add(12345);
        (self.rng_state >> 16) as f32 / 65536.0
    }

    pub fn max_hp(&self) -> i32 {
        BOSS_HP
    }

    /// Check if we should enter phase 2 (called externally after damage).
    pub fn check_phase_transition(&mut self, current_hp: i32) -> bool {
        if !self.phase2_triggered && current_hp <= PHASE2_HP_THRESHOLD && current_hp > 0 {
            self.phase2_triggered = true;
            self.phase2 = true;
            self.state = BoneKingState::Roar;
            self.timer = ROAR_DURATION;
            return true;
        }
        false
    }

    /// Start dying sequence.
    pub fn start_dying(&mut self) {
        self.state = BoneKingState::Dying;
        self.timer = DYING_DURATION;
        self.dying_flash_counter = 0.0;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        dt: f32,
        my_cx: f32,
        my_cy: f32,
        player_x: f32,
        player_y: f32,
        staggered: bool,
        alive: bool,
        wall_collision_x: bool,
        wall_collision_y: bool,
    ) -> BoneKingOutput {
        if !alive && self.state != BoneKingState::Dying {
            return BoneKingOutput::default();
        }

        // Stagger override (only in non-critical states)
        if staggered
            && !matches!(
                self.state,
                BoneKingState::Roar
                    | BoneKingState::Dying
                    | BoneKingState::ChargeActive
                    | BoneKingState::ChargeStunned
            )
        {
            // Boss resists stagger — just reduce timer slightly
        }

        self.timer -= dt;

        let dx_to_player = player_x - my_cx;
        let dy_to_player = player_y - my_cy;
        let dist = (dx_to_player * dx_to_player + dy_to_player * dy_to_player)
            .sqrt()
            .max(0.01);

        let mut output = BoneKingOutput {
            phase2: self.phase2,
            facing_right: dx_to_player > 0.0,
            ..Default::default()
        };

        let speed = if self.phase2 {
            BOSS_SPEED_PHASE2
        } else {
            BOSS_SPEED
        };
        let idle_dur = if self.phase2 {
            IDLE_DURATION_P2
        } else {
            IDLE_DURATION
        };

        match self.state {
            BoneKingState::Idle => {
                if self.timer <= 0.0 {
                    if dist > CHASE_RANGE {
                        self.state = BoneKingState::Chase;
                        self.timer = 3.0; // safety timeout
                    } else {
                        self.choose_attack(dist);
                    }
                }
            }
            BoneKingState::Chase => {
                if dist > 0.01 {
                    let nx = dx_to_player / dist;
                    let ny = dy_to_player / dist;
                    output.dx = nx * speed * dt;
                    output.dy = ny * speed * dt;
                    output.facing_right = dx_to_player > 0.0;
                }

                if dist < ATTACK_RANGE {
                    self.choose_attack(dist);
                } else if self.timer <= 0.0 {
                    // Timeout: go idle
                    self.state = BoneKingState::Idle;
                    self.timer = idle_dur;
                }
            }
            BoneKingState::SlamWindup => {
                output.telegraph = true;
                output.facing_right = self.attack_dir_x > 0.0;
                if self.timer <= 0.0 {
                    self.state = BoneKingState::SlamActive;
                    self.timer = SLAM_ACTIVE;
                    self.slam_impact_fired = false;
                }
            }
            BoneKingState::SlamActive => {
                output.facing_right = self.attack_dir_x > 0.0;
                let facing = output.facing_right;
                output.attack_hitbox = Some(self.slam_hitbox(my_cx, my_cy, facing));

                if !self.slam_impact_fired {
                    output.slam_impact = true;
                    self.slam_impact_fired = true;
                }

                if self.timer <= 0.0 {
                    self.state = BoneKingState::SlamRecover;
                    self.timer = SLAM_RECOVER;
                }
            }
            BoneKingState::SlamRecover => {
                output.facing_right = self.attack_dir_x > 0.0;
                if self.timer <= 0.0 {
                    self.state = BoneKingState::Idle;
                    self.timer = idle_dur;
                }
            }
            BoneKingState::SweepWindup => {
                output.telegraph = true;
                output.facing_right = self.attack_dir_x > 0.0;
                if self.timer <= 0.0 {
                    self.state = BoneKingState::SweepActive;
                    self.timer = SWEEP_ACTIVE;
                }
            }
            BoneKingState::SweepActive => {
                output.facing_right = self.attack_dir_x > 0.0;
                let facing = output.facing_right;
                output.attack_hitbox = Some(self.sweep_hitbox(my_cx, my_cy, facing));

                if self.timer <= 0.0 {
                    self.state = BoneKingState::SweepRecover;
                    self.timer = SWEEP_RECOVER;
                }
            }
            BoneKingState::SweepRecover => {
                output.facing_right = self.attack_dir_x > 0.0;
                if self.timer <= 0.0 {
                    self.state = BoneKingState::Idle;
                    self.timer = idle_dur;
                }
            }
            BoneKingState::ChargeWindup => {
                output.telegraph = true;
                output.facing_right = self.attack_dir_x > 0.0;
                if self.timer <= 0.0 {
                    self.state = BoneKingState::ChargeActive;
                    self.timer = CHARGE_MAX_DURATION;
                    self.charge_vx = self.attack_dir_x * CHARGE_SPEED;
                    self.charge_vy = self.attack_dir_y * CHARGE_SPEED;
                    self.charge_wall_hit_fired = false;
                }
            }
            BoneKingState::ChargeActive => {
                output.dx = self.charge_vx * dt;
                output.dy = self.charge_vy * dt;
                output.facing_right = self.charge_vx > 0.0;

                // Body hitbox during charge
                // We pass the boss sprite origin, not center, for hitbox computation
                // The hitbox is defined relative to sprite origin
                output.attack_hitbox = Some(CHARGE_HITBOX.at(
                    my_cx - 10.0, // approximate sprite origin from center
                    my_cy - 12.0,
                ));

                // Check wall collision
                if wall_collision_x || wall_collision_y {
                    if !self.charge_wall_hit_fired {
                        output.charge_wall_hit = true;
                        self.charge_wall_hit_fired = true;
                    }
                    self.state = BoneKingState::ChargeStunned;
                    self.timer = CHARGE_STUN;
                } else if self.timer <= 0.0 {
                    // Timeout without hitting a wall — go to short stun
                    self.state = BoneKingState::ChargeStunned;
                    self.timer = CHARGE_STUN * 0.5;
                }
            }
            BoneKingState::ChargeStunned => {
                output.stunned = true;
                if self.timer <= 0.0 {
                    self.state = BoneKingState::Idle;
                    self.timer = idle_dur;
                }
            }
            BoneKingState::Roar => {
                output.roaring = true;
                output.invulnerable = true;
                if self.timer <= 0.0 {
                    self.state = BoneKingState::Idle;
                    self.timer = idle_dur;
                }
            }
            BoneKingState::Dying => {
                output.dying = true;
                self.dying_flash_counter += dt * 15.0;
                if self.timer <= 0.0 {
                    output.death_finished = true;
                }
            }
        }

        output
    }

    fn choose_attack(&mut self, dist: f32) {
        // Lock attack direction
        // (direction already facing player from the caller)
        if dist > CHASE_RANGE {
            self.state = BoneKingState::Chase;
            self.timer = 3.0;
            return;
        }

        // Phase 2: 1/3 chance of charge if not already charging
        if self.phase2 && self.rand_float() < 0.33 {
            self.state = BoneKingState::ChargeWindup;
            self.timer = CHARGE_WINDUP;
            return;
        }

        if self.next_is_slam {
            self.state = BoneKingState::SlamWindup;
            self.timer = if self.phase2 {
                SLAM_WINDUP_P2
            } else {
                SLAM_WINDUP
            };
        } else {
            self.state = BoneKingState::SweepWindup;
            self.timer = if self.phase2 {
                SWEEP_WINDUP_P2
            } else {
                SWEEP_WINDUP
            };
        }
        self.next_is_slam = !self.next_is_slam;
    }

    /// Lock attack direction toward player. Call before transitioning to an attack state.
    pub fn lock_direction(&mut self, dx: f32, dy: f32) {
        let dist = (dx * dx + dy * dy).sqrt().max(0.01);
        self.attack_dir_x = dx / dist;
        self.attack_dir_y = dy / dist;
    }

    fn slam_hitbox(&self, cx: f32, cy: f32, facing_right: bool) -> AABB {
        // Convert center back to sprite origin for hitbox placement
        let ox = cx - 10.0;
        let oy = cy - 12.0;
        if facing_right {
            SLAM_HITBOX_RIGHT.at(ox, oy)
        } else {
            SLAM_HITBOX_LEFT.at(ox, oy)
        }
    }

    fn sweep_hitbox(&self, cx: f32, cy: f32, facing_right: bool) -> AABB {
        let ox = cx - 10.0;
        let oy = cy - 12.0;
        if facing_right {
            SWEEP_HITBOX_RIGHT.at(ox, oy)
        } else {
            SWEEP_HITBOX_LEFT.at(ox, oy)
        }
    }

    /// Returns the current attack hitbox in world-space, if active.
    pub fn attack_hitbox(&self, x: f32, y: f32, facing_right: bool) -> Option<AABB> {
        match self.state {
            BoneKingState::SlamActive => {
                if facing_right {
                    Some(SLAM_HITBOX_RIGHT.at(x, y))
                } else {
                    Some(SLAM_HITBOX_LEFT.at(x, y))
                }
            }
            BoneKingState::SweepActive => {
                if facing_right {
                    Some(SWEEP_HITBOX_RIGHT.at(x, y))
                } else {
                    Some(SWEEP_HITBOX_LEFT.at(x, y))
                }
            }
            BoneKingState::ChargeActive => Some(CHARGE_HITBOX.at(x, y)),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct BoneKingOutput {
    pub dx: f32,
    pub dy: f32,
    pub facing_right: bool,
    pub attack_hitbox: Option<AABB>,
    pub phase2: bool,
    pub telegraph: bool,
    pub stunned: bool,
    pub roaring: bool,
    pub dying: bool,
    pub invulnerable: bool,
    pub slam_impact: bool,
    pub charge_wall_hit: bool,
    pub death_finished: bool,
}
