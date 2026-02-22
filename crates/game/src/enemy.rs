use engine::animation::AnimationPlayer;
use engine::collision::AABB;
use engine::tilemap::TileMap;
use engine::types::Transform;
use engine::FrameBuffer;

use crate::sprites;

/// Hurtbox offset within the 10x10 enemy sprite.
const HURTBOX: AABB = AABB::new(2.0, 1.0, 6.0, 8.0);

/// Collision box at the feet for wall checks during knockback.
const COLLISION_W: f32 = 6.0;
const COLLISION_H: f32 = 4.0;
const COLLISION_OFFSET_X: f32 = 2.0;
const COLLISION_OFFSET_Y: f32 = 6.0;

const FLASH_DURATION: f32 = 0.12;
const KNOCKBACK_SPEED: f32 = 120.0;
const KNOCKBACK_FRICTION: f32 = 0.85;
const STAGGER_DURATION: f32 = 0.2;

pub struct Enemy {
    pub transform: Transform,
    pub animation: AnimationPlayer,
    pub facing_right: bool,
    pub hp: i32,
    pub alive: bool,
    pub hit_this_attack: bool,
    flash_timer: f32,
    pub knockback_vx: f32,
    pub knockback_vy: f32,
    pub stagger_timer: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            transform: Transform::new(x, y),
            animation: AnimationPlayer::new(&sprites::ENEMY_IDLE_ANIM),
            facing_right: true,
            hp: 3,
            alive: true,
            hit_this_attack: false,
            flash_timer: 0.0,
            knockback_vx: 0.0,
            knockback_vy: 0.0,
            stagger_timer: 0.0,
        }
    }

    /// Hurtbox in world coordinates.
    pub fn world_hurtbox(&self) -> AABB {
        HURTBOX.at(self.transform.position.x, self.transform.position.y)
    }

    /// Apply damage with knockback velocity and stagger.
    /// `kb_dir_x, kb_dir_y` should be a normalized direction vector.
    pub fn take_damage(&mut self, dmg: i32, kb_dir_x: f32, kb_dir_y: f32) {
        self.hp -= dmg;
        self.flash_timer = FLASH_DURATION;
        self.knockback_vx = kb_dir_x * KNOCKBACK_SPEED;
        self.knockback_vy = kb_dir_y * KNOCKBACK_SPEED;
        self.stagger_timer = STAGGER_DURATION;

        if self.hp <= 0 {
            self.alive = false;
            self.animation.play(&sprites::ENEMY_DEATH_ANIM);
        }
    }

    pub fn update(&mut self, dt: f64, tilemap: &TileMap) {
        self.transform.commit();

        let dt_f32 = dt as f32;
        if self.flash_timer > 0.0 {
            self.flash_timer -= dt_f32;
        }
        if self.stagger_timer > 0.0 {
            self.stagger_timer -= dt_f32;
        }

        // Apply knockback velocity with friction and wall collision
        if self.knockback_vx.abs() > 0.5 || self.knockback_vy.abs() > 0.5 {
            let friction = KNOCKBACK_FRICTION.powf(dt_f32 * 30.0);
            self.knockback_vx *= friction;
            self.knockback_vy *= friction;

            // Move X with wall collision
            let move_x = self.knockback_vx * dt_f32;
            let try_x = self.transform.position.x + move_x;
            if !tilemap.collides(
                try_x + COLLISION_OFFSET_X,
                self.transform.position.y + COLLISION_OFFSET_Y,
                COLLISION_W,
                COLLISION_H,
            ) {
                self.transform.position.x = try_x;
            } else {
                self.knockback_vx = 0.0;
            }

            // Move Y with wall collision
            let move_y = self.knockback_vy * dt_f32;
            let try_y = self.transform.position.y + move_y;
            if !tilemap.collides(
                self.transform.position.x + COLLISION_OFFSET_X,
                try_y + COLLISION_OFFSET_Y,
                COLLISION_W,
                COLLISION_H,
            ) {
                self.transform.position.y = try_y;
            } else {
                self.knockback_vy = 0.0;
            }
        }

        self.animation.update(dt);
    }

    pub fn render(&self, fb: &mut FrameBuffer, alpha: f32, cam_x: i32, cam_y: i32) {
        // Don't render if death animation is finished
        if !self.alive && self.animation.is_finished() {
            return;
        }

        let sprite = self.animation.current_sprite();
        let pos = self.transform.interpolated(alpha);
        let px = pos.x as i32 - cam_x;
        let py = pos.y as i32 - cam_y;

        if self.flash_timer > 0.0 {
            let flipped = self.animation.is_flipped();
            if flipped {
                fb.blit_sprite_flipped_solid(sprite, px, py, [255, 255, 255]);
            } else {
                fb.blit_sprite_solid(sprite, px, py, [255, 255, 255]);
            }
        } else {
            let flipped = self.animation.is_flipped();
            if flipped {
                fb.blit_sprite_flipped(sprite, px, py);
            } else {
                fb.blit_sprite(sprite, px, py);
            }
        }
    }
}
