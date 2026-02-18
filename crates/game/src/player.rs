use engine::animation::AnimationPlayer;
use engine::input::{GameKey, InputState};
use engine::types::Transform;
use engine::{Color, FrameBuffer};

use crate::sprites;

const PLAYER_SPEED: f32 = 60.0; // pixels per second
const DASH_SPEED: f32 = 200.0; // pixels per second
const DASH_DURATION: f32 = 0.15; // seconds

pub enum PlayerState {
    Idle,
    Walking,
    Dashing,
}

pub struct Player {
    pub transform: Transform,
    pub state: PlayerState,
    pub animation: AnimationPlayer,
    pub facing_right: bool,
    dash_timer: f32,
    dash_dx: f32,
    dash_dy: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            transform: Transform::new(x, y),
            state: PlayerState::Idle,
            animation: AnimationPlayer::new(&sprites::IDLE_ANIM),
            facing_right: true,
            dash_timer: 0.0,
            dash_dx: 0.0,
            dash_dy: 0.0,
        }
    }

    pub fn update(&mut self, input: &InputState, dt: f64) {
        self.transform.commit();

        let dt_f32 = dt as f32;
        let (dx, dy) = input.direction();

        // Update facing direction
        if dx > 0.0 {
            self.facing_right = true;
        } else if dx < 0.0 {
            self.facing_right = false;
        }

        match self.state {
            PlayerState::Dashing => {
                self.dash_timer -= dt_f32;
                self.transform.position.x += self.dash_dx * DASH_SPEED * dt_f32;
                self.transform.position.y += self.dash_dy * DASH_SPEED * dt_f32;
                if self.dash_timer <= 0.0 {
                    if dx != 0.0 || dy != 0.0 {
                        self.state = PlayerState::Walking;
                        self.animation.play(&sprites::WALK_ANIM);
                    } else {
                        self.state = PlayerState::Idle;
                        self.animation.play(&sprites::IDLE_ANIM);
                    }
                }
            }
            _ => {
                if input.is_pressed(GameKey::Dash) && (dx != 0.0 || dy != 0.0) {
                    self.state = PlayerState::Dashing;
                    self.dash_timer = DASH_DURATION;
                    self.dash_dx = dx;
                    self.dash_dy = dy;
                    self.animation.play(&sprites::DASH_ANIM);
                } else if dx != 0.0 || dy != 0.0 {
                    self.state = PlayerState::Walking;
                    self.animation.play(&sprites::WALK_ANIM);
                    self.transform.position.x += dx * PLAYER_SPEED * dt_f32;
                    self.transform.position.y += dy * PLAYER_SPEED * dt_f32;
                } else {
                    self.state = PlayerState::Idle;
                    self.animation.play(&sprites::IDLE_ANIM);
                }
            }
        }

        self.animation.set_flipped(!self.facing_right);
        self.animation.update(dt);
    }

    pub fn render(&mut self, fb: &mut FrameBuffer, alpha: f32) {
        self.render_inner(fb, alpha, None);
    }

    pub fn render_tinted(&mut self, fb: &mut FrameBuffer, alpha: f32, tint: Color) {
        self.render_inner(fb, alpha, Some(tint));
    }

    fn render_inner(&mut self, fb: &mut FrameBuffer, alpha: f32, tint: Option<Color>) {
        let sprite = self.animation.current_sprite();
        let w = fb.width() as f32;
        let h = fb.height() as f32;

        // Clamp position to screen bounds
        self.transform.position.x = self
            .transform
            .position
            .x
            .clamp(0.0, (w - sprite.width as f32).max(0.0));
        self.transform.position.y = self
            .transform
            .position
            .y
            .clamp(0.0, (h - sprite.height as f32).max(0.0));

        let pos = self.transform.interpolated(alpha);
        let px = pos.x.clamp(0.0, (w - sprite.width as f32).max(0.0)) as i32;
        let py = pos.y.clamp(0.0, (h - sprite.height as f32).max(0.0)) as i32;

        let flipped = self.animation.is_flipped();
        match (flipped, tint) {
            (false, None) => fb.blit_sprite(sprite, px, py),
            (true, None) => fb.blit_sprite_flipped(sprite, px, py),
            (false, Some(t)) => fb.blit_sprite_tinted(sprite, px, py, t),
            (true, Some(t)) => fb.blit_sprite_flipped_tinted(sprite, px, py, t),
        }
    }
}
