use engine::animation::AnimationPlayer;
use engine::input::{GameKey, InputState};
use engine::tilemap::TileMap;
use engine::types::Transform;
use engine::{Color, FrameBuffer};

use crate::sprites;

const PLAYER_SPEED: f32 = 60.0; // pixels per second
const DASH_SPEED: f32 = 200.0; // pixels per second
const DASH_DURATION: f32 = 0.15; // seconds

// Collision box: 8×4 pixels at the feet of the 10×14 sprite
const COLLISION_W: f32 = 8.0;
const COLLISION_H: f32 = 4.0;
const COLLISION_OFFSET_X: f32 = 1.0; // (10 - 8) / 2
const COLLISION_OFFSET_Y: f32 = 10.0; // 14 - 4

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

    /// Player center in world pixels (for camera following).
    pub fn center(&self) -> (f32, f32) {
        (
            self.transform.position.x + 5.0,
            self.transform.position.y + 7.0,
        )
    }

    pub fn update(&mut self, input: &InputState, dt: f64, tilemap: &TileMap) {
        self.transform.commit();

        let dt_f32 = dt as f32;
        let (dx, dy) = input.direction();

        // Update facing direction
        if dx > 0.0 {
            self.facing_right = true;
        } else if dx < 0.0 {
            self.facing_right = false;
        }

        // Determine movement based on state
        let (move_x, move_y) = match self.state {
            PlayerState::Dashing => {
                self.dash_timer -= dt_f32;
                let mx = self.dash_dx * DASH_SPEED * dt_f32;
                let my = self.dash_dy * DASH_SPEED * dt_f32;
                if self.dash_timer <= 0.0 {
                    if dx != 0.0 || dy != 0.0 {
                        self.state = PlayerState::Walking;
                        self.animation.play(&sprites::WALK_ANIM);
                    } else {
                        self.state = PlayerState::Idle;
                        self.animation.play(&sprites::IDLE_ANIM);
                    }
                }
                (mx, my)
            }
            _ => {
                if input.is_pressed(GameKey::Dash) && (dx != 0.0 || dy != 0.0) {
                    self.state = PlayerState::Dashing;
                    self.dash_timer = DASH_DURATION;
                    self.dash_dx = dx;
                    self.dash_dy = dy;
                    self.animation.play(&sprites::DASH_ANIM);
                    (dx * DASH_SPEED * dt_f32, dy * DASH_SPEED * dt_f32)
                } else if dx != 0.0 || dy != 0.0 {
                    self.state = PlayerState::Walking;
                    self.animation.play(&sprites::WALK_ANIM);
                    (dx * PLAYER_SPEED * dt_f32, dy * PLAYER_SPEED * dt_f32)
                } else {
                    self.state = PlayerState::Idle;
                    self.animation.play(&sprites::IDLE_ANIM);
                    (0.0, 0.0)
                }
            }
        };

        // Apply movement with collision (wall sliding)
        self.try_move(move_x, move_y, tilemap);

        self.animation.set_flipped(!self.facing_right);
        self.animation.update(dt);
    }

    /// Try to move by (move_x, move_y), checking X and Y independently for wall sliding.
    fn try_move(&mut self, move_x: f32, move_y: f32, tilemap: &TileMap) {
        // Try X
        let try_x = self.transform.position.x + move_x;
        if !tilemap.collides(
            try_x + COLLISION_OFFSET_X,
            self.transform.position.y + COLLISION_OFFSET_Y,
            COLLISION_W,
            COLLISION_H,
        ) {
            self.transform.position.x = try_x;
        }

        // Try Y (using potentially updated X)
        let try_y = self.transform.position.y + move_y;
        if !tilemap.collides(
            self.transform.position.x + COLLISION_OFFSET_X,
            try_y + COLLISION_OFFSET_Y,
            COLLISION_W,
            COLLISION_H,
        ) {
            self.transform.position.y = try_y;
        }
    }

    pub fn render(&mut self, fb: &mut FrameBuffer, alpha: f32, cam_x: i32, cam_y: i32) {
        self.render_inner(fb, alpha, cam_x, cam_y, None);
    }

    pub fn render_tinted(
        &mut self,
        fb: &mut FrameBuffer,
        alpha: f32,
        cam_x: i32,
        cam_y: i32,
        tint: Color,
    ) {
        self.render_inner(fb, alpha, cam_x, cam_y, Some(tint));
    }

    fn render_inner(
        &mut self,
        fb: &mut FrameBuffer,
        alpha: f32,
        cam_x: i32,
        cam_y: i32,
        tint: Option<Color>,
    ) {
        let sprite = self.animation.current_sprite();
        let pos = self.transform.interpolated(alpha);
        let px = pos.x as i32 - cam_x;
        let py = pos.y as i32 - cam_y;

        let flipped = self.animation.is_flipped();
        match (flipped, tint) {
            (false, None) => fb.blit_sprite(sprite, px, py),
            (true, None) => fb.blit_sprite_flipped(sprite, px, py),
            (false, Some(t)) => fb.blit_sprite_tinted(sprite, px, py, t),
            (true, Some(t)) => fb.blit_sprite_flipped_tinted(sprite, px, py, t),
        }
    }
}
