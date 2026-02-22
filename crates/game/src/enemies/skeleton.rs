use engine::collision::AABB;

const DETECT_RANGE: f32 = 80.0;
const ATTACK_RANGE: f32 = 20.0;
const PATROL_SPEED: f32 = 25.0;
const CHASE_SPEED: f32 = 40.0;
const LUNGE_DISTANCE: f32 = 15.0;
const WINDUP_DURATION: f32 = 0.4;
const ATTACK_DURATION: f32 = 0.15;
const COOLDOWN_DURATION: f32 = 0.6;
const STAGGER_DURATION: f32 = 0.3;
const IDLE_MIN: f32 = 1.0;
const IDLE_MAX: f32 = 2.0;

/// Skeleton attack hitbox: 10x8, extends from front of sprite
const SKEL_ATTACK_HITBOX_RIGHT: AABB = AABB::new(8.0, 3.0, 10.0, 8.0);
const SKEL_ATTACK_HITBOX_LEFT: AABB = AABB::new(-10.0, 3.0, 10.0, 8.0);

#[derive(Clone, Copy, PartialEq)]
pub enum SkeletonState {
    Idle,
    Patrol,
    Chase,
    WindUp,
    Attack,
    Cooldown,
    Stagger,
}

pub struct SkeletonAI {
    pub state: SkeletonState,
    timer: f32,
    patrol_dx: f32,
    patrol_dy: f32,
    /// Direction locked at start of wind-up for consistent attack direction
    attack_dir_x: f32,
    attack_dir_y: f32,
    /// Lunge progress during attack
    lunge_progress: f32,
    rng_state: u32,
}

impl SkeletonAI {
    pub fn new(seed: u32) -> Self {
        Self {
            state: SkeletonState::Idle,
            timer: 1.5,
            patrol_dx: 1.0,
            patrol_dy: 0.0,
            attack_dir_x: 1.0,
            attack_dir_y: 0.0,
            lunge_progress: 0.0,
            rng_state: seed,
        }
    }

    fn rand_float(&mut self) -> f32 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(1103515245)
            .wrapping_add(12345);
        (self.rng_state >> 16) as f32 / 65536.0
    }

    /// Returns (move_dx, move_dy, facing_right_hint, attack_hitbox_active)
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        dt: f32,
        my_x: f32,
        my_y: f32,
        player_x: f32,
        player_y: f32,
        staggered: bool,
        alive: bool,
    ) -> SkeletonOutput {
        if !alive {
            return SkeletonOutput::default();
        }

        // Override state if staggered from outside (damage)
        if staggered && self.state != SkeletonState::Stagger {
            self.state = SkeletonState::Stagger;
            self.timer = STAGGER_DURATION;
        }

        self.timer -= dt;

        let dx_to_player = player_x - my_x;
        let dy_to_player = player_y - my_y;
        let dist = (dx_to_player * dx_to_player + dy_to_player * dy_to_player).sqrt();

        let mut output = SkeletonOutput::default();

        match self.state {
            SkeletonState::Idle => {
                if dist < DETECT_RANGE {
                    self.state = SkeletonState::Chase;
                    self.timer = 0.0;
                } else if self.timer <= 0.0 {
                    self.state = SkeletonState::Patrol;
                    // Random patrol direction
                    let angle = self.rand_float() * std::f32::consts::TAU;
                    self.patrol_dx = angle.cos();
                    self.patrol_dy = angle.sin();
                    self.timer = 1.5 + self.rand_float() * 2.0;
                }
            }
            SkeletonState::Patrol => {
                output.move_dx = self.patrol_dx * PATROL_SPEED * dt;
                output.move_dy = self.patrol_dy * PATROL_SPEED * dt;
                if self.patrol_dx > 0.0 {
                    output.facing_right = true;
                } else if self.patrol_dx < 0.0 {
                    output.facing_right = false;
                }
                output.walking = true;

                if dist < DETECT_RANGE {
                    self.state = SkeletonState::Chase;
                    self.timer = 0.0;
                } else if self.timer <= 0.0 {
                    self.state = SkeletonState::Idle;
                    self.timer = IDLE_MIN + self.rand_float() * (IDLE_MAX - IDLE_MIN);
                }
            }
            SkeletonState::Chase => {
                if dist > 0.01 {
                    let nx = dx_to_player / dist;
                    let ny = dy_to_player / dist;
                    output.move_dx = nx * CHASE_SPEED * dt;
                    output.move_dy = ny * CHASE_SPEED * dt;
                    output.facing_right = dx_to_player > 0.0;
                    output.walking = true;
                }

                if dist < ATTACK_RANGE {
                    self.state = SkeletonState::WindUp;
                    self.timer = WINDUP_DURATION;
                    // Lock attack direction
                    if dist > 0.01 {
                        self.attack_dir_x = dx_to_player / dist;
                        self.attack_dir_y = dy_to_player / dist;
                    }
                } else if dist > DETECT_RANGE * 1.5 {
                    self.state = SkeletonState::Idle;
                    self.timer = IDLE_MIN + self.rand_float() * (IDLE_MAX - IDLE_MIN);
                }
            }
            SkeletonState::WindUp => {
                output.winding_up = true;
                output.telegraph = true;
                output.facing_right = self.attack_dir_x > 0.0;
                if self.timer <= 0.0 {
                    self.state = SkeletonState::Attack;
                    self.timer = ATTACK_DURATION;
                    self.lunge_progress = 0.0;
                }
            }
            SkeletonState::Attack => {
                output.attacking = true;
                output.facing_right = self.attack_dir_x > 0.0;

                // Lunge forward
                let lunge_speed = LUNGE_DISTANCE / ATTACK_DURATION;
                output.move_dx = self.attack_dir_x * lunge_speed * dt;
                output.move_dy = self.attack_dir_y * lunge_speed * dt;
                self.lunge_progress += dt;

                // Hitbox active during attack
                output.hitbox_active = true;

                if self.timer <= 0.0 {
                    self.state = SkeletonState::Cooldown;
                    self.timer = COOLDOWN_DURATION;
                }
            }
            SkeletonState::Cooldown => {
                if self.timer <= 0.0 {
                    if dist < DETECT_RANGE {
                        self.state = SkeletonState::Chase;
                    } else {
                        self.state = SkeletonState::Idle;
                        self.timer = IDLE_MIN + self.rand_float() * (IDLE_MAX - IDLE_MIN);
                    }
                }
            }
            SkeletonState::Stagger => {
                if self.timer <= 0.0 {
                    if dist < DETECT_RANGE {
                        self.state = SkeletonState::Chase;
                    } else {
                        self.state = SkeletonState::Idle;
                        self.timer = IDLE_MIN + self.rand_float() * (IDLE_MAX - IDLE_MIN);
                    }
                }
            }
        }

        output
    }

    /// Returns the world-space attack hitbox for this skeleton, if active.
    pub fn attack_hitbox(&self, x: f32, y: f32, facing_right: bool) -> Option<AABB> {
        if self.state != SkeletonState::Attack {
            return None;
        }
        if facing_right {
            Some(SKEL_ATTACK_HITBOX_RIGHT.at(x, y))
        } else {
            Some(SKEL_ATTACK_HITBOX_LEFT.at(x, y))
        }
    }
}

#[derive(Default)]
pub struct SkeletonOutput {
    pub move_dx: f32,
    pub move_dy: f32,
    pub facing_right: bool,
    pub walking: bool,
    pub winding_up: bool,
    pub attacking: bool,
    pub hitbox_active: bool,
    pub telegraph: bool,
}
