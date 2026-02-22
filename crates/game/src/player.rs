use engine::animation::AnimationPlayer;
use engine::collision::AABB;
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

const ATTACK_COOLDOWN: f32 = 0.3;
const ATTACK_ACTIVE_FRAME: usize = 2;
const PLAYER_HURTBOX: AABB = AABB::new(2.0, 3.0, 6.0, 8.0);
const ATTACK_HITBOX_RIGHT: AABB = AABB::new(8.0, 3.0, 10.0, 8.0);
const ATTACK_HITBOX_LEFT: AABB = AABB::new(-10.0, 3.0, 10.0, 8.0);
const PLAYER_KNOCKBACK_SPEED: f32 = 100.0;
const PLAYER_KNOCKBACK_FRICTION: f32 = 0.85;
const DAMAGE_INVINCIBILITY: f32 = 1.0;

pub enum PlayerState {
    Idle,
    Walking,
    Dashing,
    Attacking,
    Hit,
    Dead,
}

pub struct Player {
    pub transform: Transform,
    pub state: PlayerState,
    pub animation: AnimationPlayer,
    pub facing_right: bool,
    dash_timer: f32,
    dash_dx: f32,
    dash_dy: f32,
    pub attack_cooldown: f32,
    pub attack_active: bool,
    pub invincible_timer: f32,
    pub hp: i32,
    pub max_hp: i32,
    knockback_vx: f32,
    knockback_vy: f32,
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
            attack_cooldown: 0.0,
            attack_active: false,
            invincible_timer: 0.0,
            hp: 5,
            max_hp: 5,
            knockback_vx: 0.0,
            knockback_vy: 0.0,
        }
    }

    /// Player center in world pixels (for camera following).
    pub fn center(&self) -> (f32, f32) {
        (
            self.transform.position.x + 5.0,
            self.transform.position.y + 7.0,
        )
    }

    pub fn is_dashing(&self) -> bool {
        matches!(self.state, PlayerState::Dashing)
    }

    #[allow(dead_code)]
    pub fn is_attacking(&self) -> bool {
        matches!(self.state, PlayerState::Attacking)
    }

    pub fn is_dead(&self) -> bool {
        matches!(self.state, PlayerState::Dead)
    }

    pub fn is_invincible(&self) -> bool {
        self.invincible_timer > 0.0 || self.is_dashing()
    }

    /// Apply damage to the player. Returns true if the player died.
    pub fn take_damage(&mut self, dmg: i32, kb_dir_x: f32, kb_dir_y: f32) -> bool {
        if self.is_invincible() || self.is_dead() {
            return false;
        }
        self.hp -= dmg;
        self.invincible_timer = DAMAGE_INVINCIBILITY;
        self.knockback_vx = kb_dir_x * PLAYER_KNOCKBACK_SPEED;
        self.knockback_vy = kb_dir_y * PLAYER_KNOCKBACK_SPEED;

        if self.hp <= 0 {
            self.hp = 0;
            self.state = PlayerState::Dead;
            self.animation.play(&sprites::DEATH_ANIM);
            true
        } else {
            self.state = PlayerState::Hit;
            self.animation.play(&sprites::HIT_ANIM);
            false
        }
    }

    pub fn update(&mut self, input: &InputState, dt: f64, tilemap: &TileMap) {
        let (dx, dy) = input.direction();
        let attack = input.is_pressed(GameKey::Attack);
        let dash = input.is_pressed(GameKey::Dash);
        self.update_with_input(dx, dy, attack, dash, dt, tilemap);
    }

    /// Core update logic with explicit inputs (used by both normal play and demo mode).
    pub fn update_with_input(
        &mut self,
        dx: f32,
        dy: f32,
        attack: bool,
        dash: bool,
        dt: f64,
        tilemap: &TileMap,
    ) {
        self.transform.commit();

        let dt_f32 = dt as f32;

        // Apply knockback velocity (works in all states including Hit/Dead)
        if self.knockback_vx.abs() > 0.5 || self.knockback_vy.abs() > 0.5 {
            let friction = PLAYER_KNOCKBACK_FRICTION.powf(dt_f32 * 30.0);
            self.knockback_vx *= friction;
            self.knockback_vy *= friction;
            let kx = self.knockback_vx * dt_f32;
            let ky = self.knockback_vy * dt_f32;
            self.try_move(kx, ky, tilemap);
        }

        // Dead state: only update animation, no game logic
        if self.is_dead() {
            self.animation.update(dt);
            return;
        }

        // Tick cooldowns
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= dt_f32;
        }
        if self.invincible_timer > 0.0 {
            self.invincible_timer -= dt_f32;
        }

        // Update facing direction (not during attack or hit — keep facing)
        if !matches!(self.state, PlayerState::Attacking | PlayerState::Hit) {
            if dx > 0.0 {
                self.facing_right = true;
            } else if dx < 0.0 {
                self.facing_right = false;
            }
        }

        // Determine movement based on state
        let (move_x, move_y) = match self.state {
            PlayerState::Attacking => {
                // Locked in place during attack; wait for animation to finish
                if self.animation.is_finished() {
                    if dx != 0.0 || dy != 0.0 {
                        self.state = PlayerState::Walking;
                        self.animation.play(&sprites::WALK_ANIM);
                    } else {
                        self.state = PlayerState::Idle;
                        self.animation.play(&sprites::IDLE_ANIM);
                    }
                }
                (0.0, 0.0)
            }
            PlayerState::Hit => {
                if self.animation.is_finished() {
                    if dx != 0.0 || dy != 0.0 {
                        self.state = PlayerState::Walking;
                        self.animation.play(&sprites::WALK_ANIM);
                    } else {
                        self.state = PlayerState::Idle;
                        self.animation.play(&sprites::IDLE_ANIM);
                    }
                }
                (0.0, 0.0)
            }
            PlayerState::Dead => (0.0, 0.0),
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
                // Attack takes priority over dash (only if cooldown expired)
                if attack && self.attack_cooldown <= 0.0 {
                    self.state = PlayerState::Attacking;
                    self.attack_cooldown = ATTACK_COOLDOWN;
                    self.animation.play(&sprites::ATTACK_ANIM);
                    (0.0, 0.0)
                } else if dash && (dx != 0.0 || dy != 0.0) {
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

        // Track whether the attack hitbox is active this frame
        self.attack_active = matches!(self.state, PlayerState::Attacking)
            && self.animation.current_frame() == ATTACK_ACTIVE_FRAME;
    }

    /// Returns the world-space attack hitbox, only when the active frame is live.
    pub fn attack_hitbox(&self) -> Option<AABB> {
        if !self.attack_active {
            return None;
        }
        let px = self.transform.position.x;
        let py = self.transform.position.y;
        if self.facing_right {
            Some(ATTACK_HITBOX_RIGHT.at(px, py))
        } else {
            Some(ATTACK_HITBOX_LEFT.at(px, py))
        }
    }

    /// Player hurtbox in world coordinates.
    pub fn world_hurtbox(&self) -> AABB {
        PLAYER_HURTBOX.at(self.transform.position.x, self.transform.position.y)
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
