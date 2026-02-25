//! Combat system: hitbox checks, damage application, and feedback effects.

use engine::{BurstConfig, Color, ParticleSystem};

use crate::enemies::{Enemy, EnemyType};
use crate::hud;
use crate::player::Player;
use crate::projectile::ProjectileSystem;

// --- Particle burst configurations for combat feedback ---

const HIT_SPARK_COLORS: &[Color] = &[
    [255, 255, 255],
    [255, 255, 200],
    [255, 220, 100],
    [255, 180, 50],
];

const HIT_SPARK_CONFIG: BurstConfig = BurstConfig {
    count_min: 8,
    count_max: 12,
    speed_min: 30.0,
    speed_max: 80.0,
    lifetime_min: 0.15,
    lifetime_max: 0.35,
    colors: HIT_SPARK_COLORS,
    gravity: 0.0,
    friction: 0.9,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

const DEATH_COLORS: &[Color] = &[
    [255, 255, 255],
    [255, 255, 200],
    [255, 200, 100],
    [200, 255, 200],
    [100, 255, 100],
];

const DEATH_BURST_CONFIG: BurstConfig = BurstConfig {
    count_min: 20,
    count_max: 30,
    speed_min: 20.0,
    speed_max: 100.0,
    lifetime_min: 0.3,
    lifetime_max: 0.8,
    colors: DEATH_COLORS,
    gravity: 40.0,
    friction: 0.92,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

const BLOOD_COLORS: &[Color] = &[[200, 30, 30], [150, 20, 20], [255, 50, 50], [180, 10, 10]];

const BLOOD_BURST_CONFIG: BurstConfig = BurstConfig {
    count_min: 10,
    count_max: 16,
    speed_min: 20.0,
    speed_max: 70.0,
    lifetime_min: 0.2,
    lifetime_max: 0.5,
    colors: BLOOD_COLORS,
    gravity: 40.0,
    friction: 0.9,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

const PLAYER_DEATH_BURST_CONFIG: BurstConfig = BurstConfig {
    count_min: 25,
    count_max: 40,
    speed_min: 15.0,
    speed_max: 90.0,
    lifetime_min: 0.4,
    lifetime_max: 1.0,
    colors: BLOOD_COLORS,
    gravity: 30.0,
    friction: 0.92,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

// --- Boss-specific particle configs ---

const BOSS_DEATH_COLORS: &[Color] = &[
    [255, 255, 255],
    [255, 200, 100],
    [255, 100, 50],
    [200, 50, 30],
    [255, 220, 50],
    [200, 180, 150],
];

pub const BOSS_DEATH_BURST_CONFIG: BurstConfig = BurstConfig {
    count_min: 30,
    count_max: 40,
    speed_min: 20.0,
    speed_max: 120.0,
    lifetime_min: 0.4,
    lifetime_max: 1.2,
    colors: BOSS_DEATH_COLORS,
    gravity: 30.0,
    friction: 0.92,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

const BOSS_SLAM_COLORS: &[Color] = &[
    [200, 180, 150],
    [160, 140, 110],
    [120, 100, 70],
    [255, 200, 100],
];

pub const BOSS_SLAM_BURST_CONFIG: BurstConfig = BurstConfig {
    count_min: 12,
    count_max: 18,
    speed_min: 30.0,
    speed_max: 90.0,
    lifetime_min: 0.2,
    lifetime_max: 0.5,
    colors: BOSS_SLAM_COLORS,
    gravity: 50.0,
    friction: 0.88,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

pub const PROJ_TRAIL_COLORS: &[Color] = &[[60, 200, 255], [30, 100, 180], [100, 220, 255]];

pub const PROJ_TRAIL_CONFIG: BurstConfig = BurstConfig {
    count_min: 1,
    count_max: 2,
    speed_min: 3.0,
    speed_max: 10.0,
    lifetime_min: 0.1,
    lifetime_max: 0.2,
    colors: PROJ_TRAIL_COLORS,
    gravity: 0.0,
    friction: 0.8,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

const PROJ_IMPACT_COLORS: &[Color] = &[
    [60, 200, 255],
    [100, 220, 255],
    [200, 240, 255],
    [30, 100, 180],
];

const PROJ_IMPACT_CONFIG: BurstConfig = BurstConfig {
    count_min: 6,
    count_max: 10,
    speed_min: 20.0,
    speed_max: 60.0,
    lifetime_min: 0.15,
    lifetime_max: 0.3,
    colors: PROJ_IMPACT_COLORS,
    gravity: 0.0,
    friction: 0.85,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

/// Effects produced by combat checks, applied by the game loop.
#[derive(Default)]
pub struct CombatEffects {
    pub hit_pause_frames: u32,
    pub camera_shake: f32,
    pub player_died: bool,
}

/// Check player's attack hitbox against all enemy hurtboxes.
/// Applies damage to enemies, spawns hit/death particles and damage numbers.
pub fn check_player_attacks(
    player: &Player,
    enemies: &mut [Enemy],
    particles: &mut ParticleSystem,
    damage_numbers: &mut Vec<hud::DamageNumber>,
) -> CombatEffects {
    let mut effects = CombatEffects::default();

    let attack_hb = match player.attack_hitbox() {
        Some(hb) => hb,
        None => return effects,
    };

    let weapon = player.weapon();
    let dmg = weapon.base_damage;
    let kb_force = weapon.knockback_force;

    let (pcx, pcy) = player.center();
    for enemy in enemies.iter_mut() {
        if !enemy.alive || enemy.hit_this_attack {
            continue;
        }
        let hurtbox = enemy.world_hurtbox();
        if attack_hb.overlaps(&hurtbox) {
            let (ecx, ecy) = hurtbox.center();
            let dx = ecx - pcx;
            let dy = ecy - pcy;
            let len = (dx * dx + dy * dy).sqrt().max(0.01);

            let was_alive = enemy.alive;
            enemy.take_damage_with_knockback(dmg, dx / len, dy / len, kb_force);
            enemy.hit_this_attack = true;

            particles.burst(ecx, ecy, &HIT_SPARK_CONFIG);

            if !enemy.alive && was_alive {
                // Kill: bigger feedback
                effects.hit_pause_frames = 5;
                effects.camera_shake = 5.0;
                particles.burst(ecx, ecy, &DEATH_BURST_CONFIG);
                damage_numbers.push(hud::DamageNumber::new(
                    dmg,
                    ecx - 2.0,
                    ecy - 8.0,
                    [255, 80, 80],
                ));
            } else {
                // Hit: smaller feedback
                effects.hit_pause_frames = 3;
                effects.camera_shake = 2.5;
                damage_numbers.push(hud::DamageNumber::new(
                    dmg,
                    ecx - 2.0,
                    ecy - 8.0,
                    [255, 255, 100],
                ));
            }
        }
    }

    effects
}

/// Check enemy melee attacks and projectile hits against the player.
/// Applies damage to player, spawns blood/death particles and damage numbers.
pub fn check_enemy_attacks(
    player: &mut Player,
    enemies: &[Enemy],
    projectiles: &mut ProjectileSystem,
    particles: &mut ParticleSystem,
    damage_numbers: &mut Vec<hud::DamageNumber>,
) -> CombatEffects {
    let mut effects = CombatEffects::default();

    let player_hurtbox = player.world_hurtbox();
    let (pcx, pcy) = player.center();

    // Melee attacks (skeletons and boss)
    for enemy in enemies {
        if !enemy.alive {
            continue;
        }
        // Only check enemies with melee attacks
        if !matches!(
            enemy.enemy_type,
            EnemyType::Skeleton | EnemyType::BoneKing
        ) {
            continue;
        }
        if let Some(atk_hb) = enemy.attack_hitbox() {
            if atk_hb.overlaps(&player_hurtbox) {
                let (ecx, ecy) = enemy.center();
                let dx = pcx - ecx;
                let dy = pcy - ecy;
                let len = (dx * dx + dy * dy).sqrt().max(0.01);
                // Boss deals 2 damage per hit
                let dmg = if enemy.enemy_type == EnemyType::BoneKing {
                    2
                } else {
                    1
                };
                let died = player.take_damage(dmg, dx / len, dy / len);
                if died {
                    effects.hit_pause_frames = 8;
                    effects.camera_shake = 8.0;
                    effects.player_died = true;
                    particles.burst(pcx, pcy, &PLAYER_DEATH_BURST_CONFIG);
                } else if player.invincible_timer > 0.0 {
                    effects.hit_pause_frames = 4;
                    effects.camera_shake = 4.0;
                    particles.burst(pcx, pcy, &BLOOD_BURST_CONFIG);
                    damage_numbers.push(hud::DamageNumber::new(
                        dmg,
                        pcx - 2.0,
                        pcy - 8.0,
                        [255, 80, 80],
                    ));
                }
                break; // only take one melee hit per frame
            }
        }
    }

    // Projectile hits
    if !player.is_dead() {
        let proj_hits = projectiles.check_player_hits(&player.world_hurtbox());
        for (hx, hy, dmg) in proj_hits {
            let dx = pcx - hx;
            let dy = pcy - hy;
            let len = (dx * dx + dy * dy).sqrt().max(0.01);
            let died = player.take_damage(dmg, dx / len, dy / len);
            if died {
                effects.hit_pause_frames = 8;
                effects.camera_shake = 8.0;
                effects.player_died = true;
                particles.burst(pcx, pcy, &PLAYER_DEATH_BURST_CONFIG);
                break;
            } else if player.invincible_timer > 0.0 {
                effects.hit_pause_frames = 4;
                effects.camera_shake = 4.0;
                particles.burst(hx, hy, &BLOOD_BURST_CONFIG);
                damage_numbers.push(hud::DamageNumber::new(
                    dmg,
                    hx - 2.0,
                    hy - 8.0,
                    [255, 80, 80],
                ));
                break; // one hit per frame due to i-frames
            }
        }
    }

    effects
}

/// Spawn projectiles from ghost enemies that fired this frame.
pub fn spawn_enemy_projectiles(enemies: &[Enemy], projectiles: &mut ProjectileSystem) {
    for enemy in enemies {
        if enemy.fired_projectile {
            let (ex, ey) = enemy.center();
            projectiles.spawn(ex - 1.5, ey - 1.5, enemy.aim_dir_x, enemy.aim_dir_y);
        }
    }
}

/// Update projectile physics and spawn trail/impact particles.
pub fn update_projectiles(
    projectiles: &mut ProjectileSystem,
    tilemap: &engine::TileMap,
    particles: &mut ParticleSystem,
    dt: f32,
) {
    let (trail_pos, impact_pos) = projectiles.update(dt, tilemap);
    for (tx, ty) in trail_pos {
        particles.burst(tx, ty, &PROJ_TRAIL_CONFIG);
    }
    for (ix, iy) in impact_pos {
        particles.burst(ix, iy, &PROJ_IMPACT_CONFIG);
    }
}
