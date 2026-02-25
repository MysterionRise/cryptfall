pub mod bone_king;
pub mod ghost;
pub mod skeleton;

use engine::animation::AnimationPlayer;
use engine::collision::AABB;
use engine::tilemap::TileMap;
use engine::types::Transform;
use engine::FrameBuffer;

use crate::sprites;
use bone_king::{BoneKingAI, BoneKingOutput};
use ghost::{GhostAI, GhostOutput};
use skeleton::{SkeletonAI, SkeletonOutput};

const FLASH_DURATION: f32 = 0.12;
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

// --- Bone King constants (20x24 sprite) ---
const BOSS_HURTBOX: AABB = AABB::new(4.0, 4.0, 12.0, 16.0);
const BOSS_COLLISION_W: f32 = 12.0;
const BOSS_COLLISION_H: f32 = 6.0;
const BOSS_COLLISION_OFFSET_X: f32 = 4.0;
const BOSS_COLLISION_OFFSET_Y: f32 = 18.0;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    #[allow(dead_code)] // Used when slime waves are added
    Slime,
    Skeleton,
    Ghost,
    BoneKing,
}

enum AIState {
    #[allow(dead_code)] // Used when slime waves are added
    None,
    Skeleton(SkeletonAI),
    Ghost(GhostAI),
    BoneKing(BoneKingAI),
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
    // --- Boss-specific output flags (set during update, consumed by main loop) ---
    /// True on the frame slam lands (for particles/shake)
    pub boss_slam_impact: bool,
    /// True on the frame boss hits a wall during charge
    pub boss_charge_wall_hit: bool,
    /// True when boss is roaring (phase transition)
    pub boss_roaring: bool,
    /// True when boss is in dying sequence
    pub boss_dying: bool,
    /// True when boss death animation is complete
    pub boss_death_finished: bool,
    /// True when boss is in phase 2
    pub boss_phase2: bool,
    /// True when boss is in a telegraph state
    pub boss_telegraph: bool,
    /// True when boss is stunned
    pub boss_stunned: bool,
    /// True when boss is invulnerable
    pub boss_invulnerable: bool,
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
            boss_slam_impact: false,
            boss_charge_wall_hit: false,
            boss_roaring: false,
            boss_dying: false,
            boss_death_finished: false,
            boss_phase2: false,
            boss_telegraph: false,
            boss_stunned: false,
            boss_invulnerable: false,
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
            boss_slam_impact: false,
            boss_charge_wall_hit: false,
            boss_roaring: false,
            boss_dying: false,
            boss_death_finished: false,
            boss_phase2: false,
            boss_telegraph: false,
            boss_stunned: false,
            boss_invulnerable: false,
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
            boss_slam_impact: false,
            boss_charge_wall_hit: false,
            boss_roaring: false,
            boss_dying: false,
            boss_death_finished: false,
            boss_phase2: false,
            boss_telegraph: false,
            boss_stunned: false,
            boss_invulnerable: false,
        }
    }

    pub fn new_bone_king(x: f32, y: f32, seed: u32) -> Self {
        let ai = BoneKingAI::new(seed);
        Self {
            transform: Transform::new(x, y),
            animation: AnimationPlayer::new(&sprites::boss::BONE_KING_IDLE_ANIM),
            facing_right: true,
            hp: 20,
            alive: true,
            hit_this_attack: false,
            enemy_type: EnemyType::BoneKing,
            flash_timer: 0.0,
            knockback_vx: 0.0,
            knockback_vy: 0.0,
            stagger_timer: 0.0,
            ai: AIState::BoneKing(ai),
            fired_projectile: false,
            aim_dir_x: 0.0,
            aim_dir_y: 0.0,
            contact_damage_cooldown: 0.0,
            boss_slam_impact: false,
            boss_charge_wall_hit: false,
            boss_roaring: false,
            boss_dying: false,
            boss_death_finished: false,
            boss_phase2: false,
            boss_telegraph: false,
            boss_stunned: false,
            boss_invulnerable: false,
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
            EnemyType::BoneKing => (
                self.transform.position.x + 10.0,
                self.transform.position.y + 12.0,
            ),
        }
    }

    pub fn world_hurtbox(&self) -> AABB {
        let hb = match self.enemy_type {
            EnemyType::Slime => SLIME_HURTBOX,
            EnemyType::Skeleton => SKEL_HURTBOX,
            EnemyType::Ghost => GHOST_HURTBOX,
            EnemyType::BoneKing => BOSS_HURTBOX,
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
            EnemyType::BoneKing => (
                BOSS_COLLISION_OFFSET_X,
                BOSS_COLLISION_OFFSET_Y,
                BOSS_COLLISION_W,
                BOSS_COLLISION_H,
            ),
        }
    }

    pub fn take_damage_with_knockback(&mut self, dmg: i32, kb_dir_x: f32, kb_dir_y: f32, kb_force: f32) {
        // Boss invulnerability check
        if let AIState::BoneKing(ref ai) = self.ai {
            if matches!(
                ai.state,
                bone_king::BoneKingState::Roar | bone_king::BoneKingState::Dying
            ) {
                return;
            }
        }

        self.hp -= dmg;
        self.flash_timer = FLASH_DURATION;

        // Boss has reduced knockback
        if self.enemy_type == EnemyType::BoneKing {
            self.knockback_vx = kb_dir_x * kb_force * 0.3;
            self.knockback_vy = kb_dir_y * kb_force * 0.3;
            self.stagger_timer = STAGGER_DURATION * 0.5;
        } else {
            self.knockback_vx = kb_dir_x * kb_force;
            self.knockback_vy = kb_dir_y * kb_force;
            self.stagger_timer = STAGGER_DURATION;
        }

        if self.hp <= 0 {
            match self.enemy_type {
                EnemyType::BoneKing => {
                    // Boss enters dying state through AI, not instant death
                    if let AIState::BoneKing(ref mut ai) = self.ai {
                        ai.start_dying();
                    }
                }
                _ => {
                    self.alive = false;
                    match self.enemy_type {
                        EnemyType::Slime => self.animation.play(&sprites::ENEMY_DEATH_ANIM),
                        EnemyType::Skeleton => self.animation.play(&sprites::SKEL_DEATH_ANIM),
                        EnemyType::Ghost => self.animation.play(&sprites::GHOST_DEATH_ANIM),
                        EnemyType::BoneKing => unreachable!(),
                    }
                }
            }
        } else if self.enemy_type == EnemyType::BoneKing {
            // Check for phase transition
            if let AIState::BoneKing(ref mut ai) = self.ai {
                ai.check_phase_transition(self.hp);
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

    /// Returns this enemy's active attack hitbox, if any.
    pub fn attack_hitbox(&self) -> Option<AABB> {
        match &self.ai {
            AIState::Skeleton(ai) => ai.attack_hitbox(
                self.transform.position.x,
                self.transform.position.y,
                self.facing_right,
            ),
            AIState::BoneKing(ai) => ai.attack_hitbox(
                self.transform.position.x,
                self.transform.position.y,
                self.facing_right,
            ),
            _ => None,
        }
    }

    pub fn update(&mut self, dt: f64, tilemap: &TileMap, player_x: f32, player_y: f32) {
        self.transform.commit();
        let dt_f32 = dt as f32;

        self.fired_projectile = false;
        self.boss_slam_impact = false;
        self.boss_charge_wall_hit = false;
        self.boss_roaring = false;
        self.boss_dying = false;
        self.boss_death_finished = false;

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
            AIState::BoneKing(ai) => {
                // Boss update runs even during dying state
                if self.alive || ai.state == bone_king::BoneKingState::Dying {
                    let (cx, cy) = (
                        self.transform.position.x + 10.0,
                        self.transform.position.y + 12.0,
                    );

                    // Lock attack direction toward player before choosing attacks
                    ai.lock_direction(player_x - cx, player_y - cy);

                    // Try movement to detect wall collisions for charge
                    // We pass these as hints to the AI
                    let wall_x = if ai.state == bone_king::BoneKingState::ChargeActive {
                        let test_dx = if ai.state == bone_king::BoneKingState::ChargeActive {
                            // Peek at what the charge movement would be
                            let charge_speed = 100.0_f32;
                            let dir_x = if self.facing_right { 1.0 } else { -1.0 };
                            dir_x * charge_speed * dt_f32
                        } else {
                            0.0
                        };
                        let try_x = self.transform.position.x + test_dx;
                        tilemap.collides(try_x + col_ox, self.transform.position.y + col_oy, col_w, col_h)
                    } else {
                        false
                    };
                    let wall_y = false; // Charge is primarily horizontal

                    let out: BoneKingOutput = ai.update(
                        dt_f32, cx, cy, player_x, player_y,
                        self.stagger_timer > 0.0,
                        self.alive,
                        wall_x,
                        wall_y,
                    );

                    // Apply movement with wall collision
                    if (out.dx != 0.0 || out.dy != 0.0) && self.stagger_timer <= 0.0 {
                        let try_x = self.transform.position.x + out.dx;
                        if !tilemap.collides(try_x + col_ox, self.transform.position.y + col_oy, col_w, col_h) {
                            self.transform.position.x = try_x;
                        }
                        let try_y = self.transform.position.y + out.dy;
                        if !tilemap.collides(self.transform.position.x + col_ox, try_y + col_oy, col_w, col_h) {
                            self.transform.position.y = try_y;
                        }
                    }

                    self.facing_right = out.facing_right;

                    // Copy boss flags
                    self.boss_slam_impact = out.slam_impact;
                    self.boss_charge_wall_hit = out.charge_wall_hit;
                    self.boss_roaring = out.roaring;
                    self.boss_dying = out.dying;
                    self.boss_death_finished = out.death_finished;
                    self.boss_phase2 = out.phase2;
                    self.boss_telegraph = out.telegraph;
                    self.boss_stunned = out.stunned;
                    self.boss_invulnerable = out.invulnerable;

                    if out.death_finished {
                        self.alive = false;
                    }

                    // Animation selection based on state
                    if out.dying {
                        // Use stunned sprite for dying (flashing effect handled in render)
                        self.animation.play(&sprites::boss::BONE_KING_STUNNED_ANIM);
                    } else if out.roaring {
                        self.animation.play(&sprites::boss::BONE_KING_ROAR_ANIM);
                    } else if out.stunned || self.stagger_timer > 0.0 {
                        self.animation.play(&sprites::boss::BONE_KING_STUNNED_ANIM);
                    } else {
                        match ai.state {
                            bone_king::BoneKingState::SlamWindup | bone_king::BoneKingState::SlamActive => {
                                self.animation.play(&sprites::boss::BONE_KING_SLAM_ANIM);
                            }
                            bone_king::BoneKingState::SweepWindup | bone_king::BoneKingState::SweepActive => {
                                self.animation.play(&sprites::boss::BONE_KING_SWEEP_ANIM);
                            }
                            bone_king::BoneKingState::ChargeWindup | bone_king::BoneKingState::ChargeActive => {
                                self.animation.play(&sprites::boss::BONE_KING_CHARGE_ANIM);
                            }
                            bone_king::BoneKingState::Chase => {
                                self.animation.play(&sprites::boss::BONE_KING_IDLE_ANIM);
                            }
                            _ => {
                                self.animation.play(&sprites::boss::BONE_KING_IDLE_ANIM);
                            }
                        }
                    }
                }
            }
            AIState::None => {}
        }

        self.animation.set_flipped(!self.facing_right);
        self.animation.update(dt);
    }

    pub fn render(&self, fb: &mut FrameBuffer, alpha: f32, cam_x: i32, cam_y: i32) {
        // Boss dying: keep rendering until death_finished
        if self.enemy_type == EnemyType::BoneKing {
            if !self.alive && !self.boss_dying {
                return;
            }
        } else if !self.alive && self.animation.is_finished() {
            return;
        }

        let sprite = self.animation.current_sprite();
        let pos = self.transform.interpolated(alpha);
        let px = pos.x as i32 - cam_x;
        let py = pos.y as i32 - cam_y;
        let flipped = self.animation.is_flipped();

        // Determine tint for this frame
        let tint: Option<[u8; 3]> = if self.flash_timer > 0.0 {
            Some([255, 255, 255])
        } else if self.boss_dying {
            // Rapid flashing during death: alternate white/red
            if let AIState::BoneKing(ref ai) = self.ai {
                if (ai.dying_flash_counter as u32).is_multiple_of(2) {
                    Some([255, 80, 80])
                } else {
                    Some([255, 255, 255])
                }
            } else {
                None
            }
        } else if self.boss_phase2 && self.enemy_type == EnemyType::BoneKing {
            // Phase 2: red tint
            Some([255, 100, 100])
        } else {
            None
        };

        match (flipped, tint) {
            (false, None) => fb.blit_sprite(sprite, px, py),
            (true, None) => fb.blit_sprite_flipped(sprite, px, py),
            (false, Some(t)) => {
                if self.flash_timer > 0.0 {
                    fb.blit_sprite_solid(sprite, px, py, t);
                } else {
                    fb.blit_sprite_tinted(sprite, px, py, t);
                }
            }
            (true, Some(t)) => {
                if self.flash_timer > 0.0 {
                    fb.blit_sprite_flipped_solid(sprite, px, py, t);
                } else {
                    fb.blit_sprite_flipped_tinted(sprite, px, py, t);
                }
            }
        }
    }

    /// Returns the boss max HP, if this enemy is a boss.
    pub fn boss_max_hp(&self) -> Option<i32> {
        if let AIState::BoneKing(ref ai) = self.ai {
            Some(ai.max_hp())
        } else {
            None
        }
    }
}
