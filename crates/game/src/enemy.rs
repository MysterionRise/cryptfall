use engine::animation::AnimationPlayer;
use engine::collision::AABB;
use engine::tilemap::TileMap;
use engine::types::Transform;
use engine::FrameBuffer;

use crate::sprites;

/// Hurtbox offset within the 10×10 enemy sprite.
const HURTBOX: AABB = AABB::new(2.0, 1.0, 6.0, 8.0);

/// Collision box at the feet for wall checks during knockback.
const COLLISION_W: f32 = 6.0;
const COLLISION_H: f32 = 4.0;
const COLLISION_OFFSET_X: f32 = 2.0;
const COLLISION_OFFSET_Y: f32 = 6.0;

const FLASH_DURATION: f32 = 0.12;

#[allow(dead_code)]
pub struct Enemy {
    pub transform: Transform,
    pub animation: AnimationPlayer,
    pub facing_right: bool,
    pub hp: i32,
    pub alive: bool,
    pub hit_this_attack: bool,
    flash_timer: f32,
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
        }
    }

    /// Hurtbox in world coordinates.
    pub fn world_hurtbox(&self) -> AABB {
        HURTBOX.at(self.transform.position.x, self.transform.position.y)
    }

    /// Apply damage, knockback, and trigger flash/death.
    pub fn take_damage(&mut self, dmg: i32, kb_dx: f32, kb_dy: f32, tilemap: &TileMap) {
        self.hp -= dmg;
        self.flash_timer = FLASH_DURATION;

        // Apply knockback with wall collision (independent axes)
        let try_x = self.transform.position.x + kb_dx;
        if !tilemap.collides(
            try_x + COLLISION_OFFSET_X,
            self.transform.position.y + COLLISION_OFFSET_Y,
            COLLISION_W,
            COLLISION_H,
        ) {
            self.transform.position.x = try_x;
        }

        let try_y = self.transform.position.y + kb_dy;
        if !tilemap.collides(
            self.transform.position.x + COLLISION_OFFSET_X,
            try_y + COLLISION_OFFSET_Y,
            COLLISION_W,
            COLLISION_H,
        ) {
            self.transform.position.y = try_y;
        }

        if self.hp <= 0 {
            self.alive = false;
            self.animation.play(&sprites::ENEMY_DEATH_ANIM);
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.transform.commit();

        let dt_f32 = dt as f32;
        if self.flash_timer > 0.0 {
            self.flash_timer -= dt_f32;
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
            // White flash on hit
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
