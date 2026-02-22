pub mod ghost;
pub mod skeleton;

use engine::animation::AnimationPlayer;
use engine::collision::AABB;
use engine::tilemap::TileMap;
use engine::types::Transform;
use engine::FrameBuffer;

use crate::sprites;
use ghost::{GhostAI, GhostOutput};
use skeleton::{SkeletonAI, SkeletonOutput};

const FLASH_DURATION: f32 = 0.12;
const KNOCKBACK_SPEED: f32 = 120.0;
const KNOCKBACK_FRICTION: f32 = 0.85;
const STAGGER_DURATION: f32 = 0.2;

// --- Slime constants (10x10 sprite) ---
const SLIME_HURTBOX: AABB = AABB::new(2.0, 1.0, 6.0, 8.0);
const SLIME_COLLISION_W: f32 = 6.0;
const SLIME_COLLISION_H: f32 = 4.0;
const SLIME_COLLISION_OFFSET_X: f32 = 2.0;
const SLIME_COLLISION_OFFSET_Y: f32 = 6.0;

// --- Skeleton constants (10x14 sprite) ---
const SKEL_HURTBOX: AABB = AABB::new(2.0, 3.0, 6.0, 8.0);
const SKEL_COLLISION_W: f32 = 8.0;
const SKEL_COLLISION_H: f32 = 4.0;
const SKEL_COLLISION_OFFSET_X: f32 = 1.0;
const SKEL_COLLISION_OFFSET_Y: f32 = 10.0;

// --- Ghost constants (10x12 sprite) ---
const GHOST_HURTBOX: AABB = AABB::new(2.0, 2.0, 6.0, 7.0);
const GHOST_COLLISION_W: f32 = 6.0;
const GHOST_COLLISION_H: f32 = 4.0;
const GHOST_COLLISION_OFFSET_X: f32 = 2.0;
const GHOST_COLLISION_OFFSET_Y: f32 = 6.0;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    #[allow(dead_code)] // Used when slime waves are added
    Slime,
    Skeleton,
    Ghost,
}

enum AIState {
    #[allow(dead_code)] // Used when slime waves are added
    None,
    Skeleton(SkeletonAI),
    Ghost(GhostAI),
}

pub struct Enemy {
    pub transform: Transform,
    pub animation: AnimationPlayer,
    pub facing_right: bool,
    pub hp: i32,
    pub alive: bool,
    pub hit_this_attack: bool,
    pub enemy_type: EnemyType,
    flash_timer: f32,
    pub knockback_vx: f32,
    pub knockback_vy: f32,
    pub stagger_timer: f32,
    ai: AIState,
    /// Set to true on the frame ghost fires (consumed by main loop)
    pub fired_projectile: bool,
    /// Cached aim direction for projectile spawning
    pub aim_dir_x: f32,
    pub aim_dir_y: f32,
    /// Cooldown before this enemy can deal contact damage again
    pub contact_damage_cooldown: f32,
}

impl Enemy {
    #[allow(dead_code)] // Used when slime waves are added
    pub fn new_slime(x: f32, y: f32) -> Self {
        Self {
            transform: Transform::new(x, y),
            animation: AnimationPlayer::new(&sprites::ENEMY_IDLE_ANIM),
            facing_right: true,
            hp: 3,
            alive: true,
            hit_this_attack: false,
            enemy_type: EnemyType::Slime,
            flash_timer: 0.0,
            knockback_vx: 0.0,
            knockback_vy: 0.0,
            stagger_timer: 0.0,
            ai: AIState::None,
            fired_projectile: false,
            aim_dir_x: 0.0,
            aim_dir_y: 0.0,
            contact_damage_cooldown: 0.0,
        }
    }

    pub fn new_skeleton(x: f32, y: f32, seed: u32) -> Self {
        Self {
            transform: Transform::new(x, y),
            animation: AnimationPlayer::new(&sprites::SKEL_IDLE_ANIM),
            facing_right: true,
            hp: 3,
            alive: true,
            hit_this_attack: false,
            enemy_type: EnemyType::Skeleton,
            flash_timer: 0.0,
            knockback_vx: 0.0,
            knockback_vy: 0.0,
            stagger_timer: 0.0,
            ai: AIState::Skeleton(SkeletonAI::new(seed)),
            fired_projectile: false,
            aim_dir_x: 0.0,
            aim_dir_y: 0.0,
            contact_damage_cooldown: 0.0,
        }
    }

    pub fn new_ghost(x: f32, y: f32, seed: u32) -> Self {
        Self {
            transform: Transform::new(x, y),
            animation: AnimationPlayer::new(&sprites::GHOST_IDLE_ANIM),
            facing_right: true,
            hp: 2,
            alive: true,
            hit_this_attack: false,
            enemy_type: EnemyType::Ghost,
            flash_timer: 0.0,
            knockback_vx: 0.0,
            knockback_vy: 0.0,
            stagger_timer: 0.0,
            ai: AIState::Ghost(GhostAI::new(seed)),
            fired_projectile: false,
            aim_dir_x: 0.0,
            aim_dir_y: 0.0,
            contact_damage_cooldown: 0.0,
        }
    }

    pub fn center(&self) -> (f32, f32) {
        match self.enemy_type {
            EnemyType::Slime => (
                self.transform.position.x + 5.0,
                self.transform.position.y + 5.0,
            ),
            EnemyType::Skeleton => (
                self.transform.position.x + 5.0,
                self.transform.position.y + 7.0,
            ),
            EnemyType::Ghost => (
                self.transform.position.x + 5.0,
                self.transform.position.y + 6.0,
            ),
        }
    }

    pub fn world_hurtbox(&self) -> AABB {
        let hb = match self.enemy_type {
            EnemyType::Slime => SLIME_HURTBOX,
            EnemyType::Skeleton => SKEL_HURTBOX,
            EnemyType::Ghost => GHOST_HURTBOX,
        };
        hb.at(self.transform.position.x, self.transform.position.y)
    }

    fn collision_params(&self) -> (f32, f32, f32, f32) {
        match self.enemy_type {
            EnemyType::Slime => (
                SLIME_COLLISION_OFFSET_X,
                SLIME_COLLISION_OFFSET_Y,
                SLIME_COLLISION_W,
                SLIME_COLLISION_H,
            ),
            EnemyType::Skeleton => (
                SKEL_COLLISION_OFFSET_X,
                SKEL_COLLISION_OFFSET_Y,
                SKEL_COLLISION_W,
                SKEL_COLLISION_H,
            ),
            EnemyType::Ghost => (
                GHOST_COLLISION_OFFSET_X,
                GHOST_COLLISION_OFFSET_Y,
                GHOST_COLLISION_W,
                GHOST_COLLISION_H,
            ),
        }
    }

    pub fn take_damage(&mut self, dmg: i32, kb_dir_x: f32, kb_dir_y: f32) {
        self.hp -= dmg;
        self.flash_timer = FLASH_DURATION;
        self.knockback_vx = kb_dir_x * KNOCKBACK_SPEED;
        self.knockback_vy = kb_dir_y * KNOCKBACK_SPEED;
        self.stagger_timer = STAGGER_DURATION;

        if self.hp <= 0 {
            self.alive = false;
            match self.enemy_type {
                EnemyType::Slime => self.animation.play(&sprites::ENEMY_DEATH_ANIM),
                EnemyType::Skeleton => self.animation.play(&sprites::SKEL_DEATH_ANIM),
                EnemyType::Ghost => self.animation.play(&sprites::GHOST_DEATH_ANIM),
            }
        }
    }

    /// Returns true if this enemy can deal contact damage (alive Slime with cooldown expired).
    #[allow(dead_code)] // Used when slime waves are added
    pub fn can_deal_contact_damage(&self) -> bool {
        self.alive && self.enemy_type == EnemyType::Slime && self.contact_damage_cooldown <= 0.0
    }

    /// Puts contact damage on cooldown (0.5s).
    #[allow(dead_code)] // Used when slime waves are added
    pub fn apply_contact_damage_cooldown(&mut self) {
        self.contact_damage_cooldown = 0.5;
    }

    /// Returns the skeleton's attack hitbox if it has one active, else None.
    pub fn attack_hitbox(&self) -> Option<AABB> {
        if let AIState::Skeleton(ref ai) = self.ai {
            ai.attack_hitbox(
                self.transform.position.x,
                self.transform.position.y,
                self.facing_right,
            )
        } else {
            None
        }
    }

    pub fn update(&mut self, dt: f64, tilemap: &TileMap, player_x: f32, player_y: f32) {
        self.transform.commit();
        let dt_f32 = dt as f32;

        self.fired_projectile = false;

        if self.flash_timer > 0.0 {
            self.flash_timer -= dt_f32;
        }
        if self.stagger_timer > 0.0 {
            self.stagger_timer -= dt_f32;
        }
        if self.contact_damage_cooldown > 0.0 {
            self.contact_damage_cooldown -= dt_f32;
        }

        let (col_ox, col_oy, col_w, col_h) = self.collision_params();

        // Apply knockback velocity
        if self.knockback_vx.abs() > 0.5 || self.knockback_vy.abs() > 0.5 {
            let friction = KNOCKBACK_FRICTION.powf(dt_f32 * 30.0);
            self.knockback_vx *= friction;
            self.knockback_vy *= friction;

            let move_x = self.knockback_vx * dt_f32;
            let try_x = self.transform.position.x + move_x;
            if !tilemap.collides(try_x + col_ox, self.transform.position.y + col_oy, col_w, col_h)
            {
                self.transform.position.x = try_x;
            } else {
                self.knockback_vx = 0.0;
            }

            let move_y = self.knockback_vy * dt_f32;
            let try_y = self.transform.position.y + move_y;
            if !tilemap.collides(self.transform.position.x + col_ox, try_y + col_oy, col_w, col_h)
            {
                self.transform.position.y = try_y;
            } else {
                self.knockback_vy = 0.0;
            }
        }

        // Type-specific AI
        match &mut self.ai {
            AIState::Skeleton(ai) => {
                if self.alive {
                    let (cx, cy) = (
                        self.transform.position.x + 5.0,
                        self.transform.position.y + 7.0,
                    );
                    let out: SkeletonOutput = ai.update(
                        dt_f32, cx, cy, player_x, player_y,
                        self.stagger_timer > 0.0,
                        self.alive,
                    );

                    if (out.move_dx != 0.0 || out.move_dy != 0.0) && self.stagger_timer <= 0.0 {
                        let try_x = self.transform.position.x + out.move_dx;
                        if !tilemap.collides(try_x + col_ox, self.transform.position.y + col_oy, col_w, col_h) {
                            self.transform.position.x = try_x;
                        }
                        let try_y = self.transform.position.y + out.move_dy;
                        if !tilemap.collides(self.transform.position.x + col_ox, try_y + col_oy, col_w, col_h) {
                            self.transform.position.y = try_y;
                        }
                    }

                    if out.move_dx != 0.0 || out.winding_up || out.attacking {
                        self.facing_right = out.facing_right;
                    }

                    if self.stagger_timer > 0.0 {
                        self.animation.play(&sprites::SKEL_STAGGER_ANIM);
                    } else if out.attacking {
                        self.animation.play(&sprites::SKEL_ATTACK_ANIM);
                    } else if out.winding_up {
                        self.animation.play(&sprites::SKEL_WINDUP_ANIM);
                    } else if out.walking {
                        self.animation.play(&sprites::SKEL_WALK_ANIM);
                    } else {
                        self.animation.play(&sprites::SKEL_IDLE_ANIM);
                    }
                }
            }
            AIState::Ghost(ai) => {
                if self.alive {
                    let (cx, cy) = (
                        self.transform.position.x + 5.0,
                        self.transform.position.y + 6.0,
                    );
                    let out: GhostOutput = ai.update(
                        dt_f32, cx, cy, player_x, player_y,
                        self.stagger_timer > 0.0,
                        self.alive,
                    );

                    if (out.move_dx != 0.0 || out.move_dy != 0.0) && self.stagger_timer <= 0.0 {
                        let try_x = self.transform.position.x + out.move_dx;
                        if !tilemap.collides(try_x + col_ox, self.transform.position.y + col_oy, col_w, col_h) {
                            self.transform.position.x = try_x;
                        }
                        let try_y = self.transform.position.y + out.move_dy;
                        if !tilemap.collides(self.transform.position.x + col_ox, try_y + col_oy, col_w, col_h) {
                            self.transform.position.y = try_y;
                        }
                    }

                    self.facing_right = out.facing_right;

                    if out.fire_projectile {
                        self.fired_projectile = true;
                        self.aim_dir_x = ai.aim_dir_x;
                        self.aim_dir_y = ai.aim_dir_y;
                    }

                    if self.stagger_timer > 0.0 {
                        self.animation.play(&sprites::GHOST_STAGGER_ANIM);
                    } else if out.aiming {
                        self.animation.play(&sprites::GHOST_AIM_ANIM);
                    } else {
                        self.animation.play(&sprites::GHOST_IDLE_ANIM);
                    }
                }
            }
            AIState::None => {}
        }

        self.animation.set_flipped(!self.facing_right);
        self.animation.update(dt);
    }

    pub fn render(&self, fb: &mut FrameBuffer, alpha: f32, cam_x: i32, cam_y: i32) {
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
