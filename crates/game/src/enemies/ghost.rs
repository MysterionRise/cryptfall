const PREFERRED_DIST: f32 = 60.0;
const TOO_CLOSE: f32 = 40.0;
const REPOSITION_SPEED: f32 = 35.0;
const AIM_DURATION: f32 = 0.6;
const SHOOT_COOLDOWN: f32 = 1.2;
const STAGGER_DURATION: f32 = 0.3;
const AIM_CANCEL_DISTANCE: f32 = 28.0;
const MAX_AIM_RANGE: f32 = 120.0;

#[derive(Clone, Copy, PartialEq)]
pub enum GhostState {
    Float,
    Reposition,
    Aim,
    Shoot,
    Cooldown,
    Stagger,
}

pub struct GhostAI {
    pub state: GhostState,
    timer: f32,
    /// Direction locked when aiming
    pub aim_dir_x: f32,
    pub aim_dir_y: f32,
    reposition_dx: f32,
    reposition_dy: f32,
    rng_state: u32,
}

impl GhostAI {
    pub fn new(seed: u32) -> Self {
        Self {
            state: GhostState::Float,
            timer: 1.0,
            aim_dir_x: 0.0,
            aim_dir_y: 0.0,
            reposition_dx: 0.0,
            reposition_dy: 0.0,
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
    ) -> GhostOutput {
        if !alive {
            return GhostOutput::default();
        }

        if staggered && self.state != GhostState::Stagger {
            self.state = GhostState::Stagger;
            self.timer = STAGGER_DURATION;
        }

        self.timer -= dt;

        let dx = player_x - my_x;
        let dy = player_y - my_y;
        let dist = (dx * dx + dy * dy).sqrt().max(0.01);

        let mut output = GhostOutput::default();

        match self.state {
            GhostState::Float => {
                // Gentle hover. If too close, reposition away.
                if dist < TOO_CLOSE {
                    self.state = GhostState::Reposition;
                    // Move away from player
                    self.reposition_dx = -dx / dist;
                    self.reposition_dy = -dy / dist;
                    self.timer = 0.8 + self.rand_float() * 0.5;
                } else if self.timer <= 0.0 {
                    // Start aiming
                    self.state = GhostState::Aim;
                    self.timer = AIM_DURATION;
                    // Lock aim direction
                    self.aim_dir_x = dx / dist;
                    self.aim_dir_y = dy / dist;
                }

                output.facing_right = dx > 0.0;
            }
            GhostState::Reposition => {
                output.move_dx = self.reposition_dx * REPOSITION_SPEED * dt;
                output.move_dy = self.reposition_dy * REPOSITION_SPEED * dt;
                output.facing_right = dx > 0.0;

                if self.timer <= 0.0 || dist > PREFERRED_DIST {
                    self.state = GhostState::Float;
                    self.timer = 0.5 + self.rand_float() * 0.5;
                }
            }
            GhostState::Aim => {
                output.aiming = true;
                output.facing_right = self.aim_dir_x > 0.0;

                // If player gets too close during aim, cancel and reposition
                if dist < AIM_CANCEL_DISTANCE {
                    self.state = GhostState::Reposition;
                    self.reposition_dx = -dx / dist;
                    self.reposition_dy = -dy / dist;
                    self.timer = 0.6;
                } else if dist > MAX_AIM_RANGE {
                    // Player moved out of aim range, cancel and return to float
                    self.state = GhostState::Float;
                    self.timer = 0.5 + self.rand_float() * 0.5;
                } else if self.timer <= 0.0 {
                    self.state = GhostState::Shoot;
                    self.timer = 0.0;
                    output.fire_projectile = true;
                }
            }
            GhostState::Shoot => {
                self.state = GhostState::Cooldown;
                self.timer = SHOOT_COOLDOWN;
            }
            GhostState::Cooldown => {
                output.facing_right = dx > 0.0;

                if dist < TOO_CLOSE {
                    self.state = GhostState::Reposition;
                    self.reposition_dx = -dx / dist;
                    self.reposition_dy = -dy / dist;
                    self.timer = 0.8;
                } else if self.timer <= 0.0 {
                    self.state = GhostState::Float;
                    self.timer = 0.3 + self.rand_float() * 0.5;
                }
            }
            GhostState::Stagger => {
                if self.timer <= 0.0 {
                    self.state = GhostState::Float;
                    self.timer = 0.5 + self.rand_float() * 0.3;
                }
            }
        }

        output
    }
}

#[derive(Default)]
pub struct GhostOutput {
    pub move_dx: f32,
    pub move_dy: f32,
    pub facing_right: bool,
    pub aiming: bool,
    pub fire_projectile: bool,
}
