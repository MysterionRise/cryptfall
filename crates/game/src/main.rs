mod enemies;
mod hud;
mod player;
mod projectile;
mod sprites;
mod tiles;

use enemies::{Enemy, EnemyType};
use engine::{
    color, render_tilemap, BurstConfig, Camera, Color, FrameBuffer, FrameInfo, Game, GameKey,
    InputState, ParticleSystem, TileMap, TileType,
};
use player::Player;

const FLASH_FRAMES: u32 = 5;
const DEMO_IDLE_THRESHOLD: f32 = 5.0;
const DEATH_FADE_DURATION: f32 = 1.5;

/// Dash i-frames tint: cool blue
const DASH_TINT: Color = [100, 160, 255];
/// Attack hit flash tint: warm red
const ATTACK_TINT: Color = [255, 80, 80];
/// I-frame flash tint
const IFRAME_TINT: Color = [255, 255, 255];

// --- Particle burst configurations ---

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

const DASH_TRAIL_COLORS: &[Color] = &[[100, 160, 255], [150, 200, 255], [200, 230, 255]];

const DASH_TRAIL_CONFIG: BurstConfig = BurstConfig {
    count_min: 2,
    count_max: 3,
    speed_min: 5.0,
    speed_max: 15.0,
    lifetime_min: 0.1,
    lifetime_max: 0.25,
    colors: DASH_TRAIL_COLORS,
    gravity: 0.0,
    friction: 0.8,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

const DUST_PUFF_COLORS: &[Color] = &[[120, 100, 70], [140, 120, 90], [100, 80, 60]];

const DUST_PUFF_CONFIG: BurstConfig = BurstConfig {
    count_min: 4,
    count_max: 6,
    speed_min: 10.0,
    speed_max: 25.0,
    lifetime_min: 0.2,
    lifetime_max: 0.4,
    colors: DUST_PUFF_COLORS,
    gravity: -10.0,
    friction: 0.85,
    angle_spread: std::f32::consts::PI,
    base_angle: -std::f32::consts::FRAC_PI_2,
};

const PROJ_TRAIL_COLORS: &[Color] = &[[60, 200, 255], [30, 100, 180], [100, 220, 255]];

const PROJ_TRAIL_CONFIG: BurstConfig = BurstConfig {
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

struct DemoState {
    timer: f32,
    dx: f32,
    dy: f32,
    attack_timer: f32,
    seed: u32,
}

impl DemoState {
    fn new() -> Self {
        Self {
            timer: 0.0,
            dx: 1.0,
            dy: 0.0,
            attack_timer: 2.0,
            seed: 54321,
        }
    }

    fn next_random(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed >> 16
    }

    fn update(&mut self, dt: f32) -> (f32, f32, bool, bool) {
        self.timer -= dt;
        self.attack_timer -= dt;

        let attack = if self.attack_timer <= 0.0 {
            self.attack_timer = 1.5 + (self.next_random() % 30) as f32 * 0.1;
            true
        } else {
            false
        };

        if self.timer <= 0.0 {
            self.timer = 0.8 + (self.next_random() % 20) as f32 * 0.1;
            let dir = self.next_random() % 9;
            let (dx, dy) = match dir {
                0 => (1.0, 0.0),
                1 => (-1.0, 0.0),
                2 => (0.0, 1.0),
                3 => (0.0, -1.0),
                4 => (FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                5 => (-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                6 => (FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                7 => (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                _ => (0.0, 0.0),
            };
            self.dx = dx;
            self.dy = dy;
        }

        let dash = self.timer > 0.0
            && self.timer < dt
            && (self.dx != 0.0 || self.dy != 0.0)
            && self.next_random().is_multiple_of(5);

        (self.dx, self.dy, attack, dash)
    }
}

const FRAC_1_SQRT_2: f32 = std::f32::consts::FRAC_1_SQRT_2;

enum DeathPhase {
    Alive,
    Dying,
    FadeOut,
    Dead,
}

struct CryptfallGame {
    player: Player,
    enemies: Vec<Enemy>,
    projectiles: projectile::ProjectileSystem,
    tilemap: TileMap,
    camera: Camera,
    particles: ParticleSystem,
    damage_numbers: Vec<hud::DamageNumber>,
    flash_timer: u32,
    hit_pause_frames: u32,
    idle_timer: f32,
    demo: Option<DemoState>,
    debug_hitboxes: bool,
    death_phase: DeathPhase,
    death_timer: f32,
}

impl CryptfallGame {
    fn new() -> Self {
        let tilemap = create_test_room();
        let player = Player::new(120.0, 88.0);

        let mut camera = Camera::new(80, 48);
        let (cx, cy) = player.center();
        camera.follow(cx, cy);
        camera.snap();
        camera.clamp_to_bounds(tilemap.pixel_width() as f32, tilemap.pixel_height() as f32);

        let enemies = vec![
            Enemy::new_skeleton(160.0, 120.0, 11111),
            Enemy::new_skeleton(120.0, 150.0, 22222),
            Enemy::new_skeleton(60.0, 100.0, 33333),
            Enemy::new_ghost(180.0, 80.0, 44444),
            Enemy::new_ghost(100.0, 60.0, 55555),
        ];

        Self {
            player,
            enemies,
            projectiles: projectile::ProjectileSystem::new(),
            tilemap,
            camera,
            particles: ParticleSystem::new(),
            damage_numbers: Vec::new(),
            flash_timer: 0,
            hit_pause_frames: 0,
            idle_timer: 0.0,
            demo: None,
            debug_hitboxes: false,
            death_phase: DeathPhase::Alive,
            death_timer: 0.0,
        }
    }
}

fn has_input(input: &InputState) -> bool {
    input.is_held(GameKey::Up)
        || input.is_held(GameKey::Down)
        || input.is_held(GameKey::Left)
        || input.is_held(GameKey::Right)
        || input.is_pressed(GameKey::Attack)
        || input.is_pressed(GameKey::Dash)
}

impl Game for CryptfallGame {
    fn update(&mut self, input: &InputState, dt: f64) -> bool {
        if input.is_pressed(GameKey::Quit) {
            return false;
        }

        // Toggle debug hitbox display
        if input.is_pressed(GameKey::Pause) {
            self.debug_hitboxes = !self.debug_hitboxes;
        }

        let dt_f32 = dt as f32;

        // Hit pause: freeze all game logic
        if self.hit_pause_frames > 0 {
            self.hit_pause_frames -= 1;
            // Still update particles during hit pause for visual effect
            self.particles.update(dt_f32);
            return true;
        }

        // Death sequence management
        match self.death_phase {
            DeathPhase::Alive => {}
            DeathPhase::Dying => {
                // Wait for death animation to finish
                self.death_timer += dt_f32;
                self.player.update_with_input(0.0, 0.0, false, false, dt, &self.tilemap);
                self.particles.update(dt_f32);
                self.camera.update(dt);
                if self.player.animation.is_finished() {
                    self.death_phase = DeathPhase::FadeOut;
                    self.death_timer = 0.0;
                }
                return true;
            }
            DeathPhase::FadeOut => {
                self.death_timer += dt_f32;
                self.particles.update(dt_f32);
                self.camera.update(dt);
                if self.death_timer >= DEATH_FADE_DURATION {
                    self.death_phase = DeathPhase::Dead;
                }
                return true;
            }
            DeathPhase::Dead => {
                // Wait for attack key to restart
                if input.is_pressed(GameKey::Attack) {
                    self.restart();
                }
                return true;
            }
        }

        // Demo mode management
        if has_input(input) {
            self.idle_timer = 0.0;
            self.demo = None;
        } else {
            self.idle_timer += dt_f32;
        }

        let was_attacking = matches!(self.player.state, player::PlayerState::Attacking);
        let was_dashing = self.player.is_dashing();

        if self.demo.is_some() || self.idle_timer >= DEMO_IDLE_THRESHOLD {
            let demo = self.demo.get_or_insert_with(DemoState::new);
            let (dx, dy, attack, dash) = demo.update(dt_f32);
            self.player
                .update_with_input(dx, dy, attack, dash, dt, &self.tilemap);

            if attack && self.player.attack_cooldown > 0.0 {
                self.flash_timer = FLASH_FRAMES;
                self.camera.shake(3.0);
            }
            if dash {
                self.camera.shake(6.0);
            }
        } else {
            self.player.update(input, dt, &self.tilemap);

            if input.is_pressed(GameKey::Attack) && self.player.attack_cooldown > 0.0 {
                self.flash_timer = FLASH_FRAMES;
                self.camera.shake(3.0);
            }

            if input.is_pressed(GameKey::Dash) {
                self.camera.shake(6.0);
            }
        }

        // Dash trail particles
        if self.player.is_dashing() {
            let (cx, cy) = self.player.center();
            self.particles.burst(cx, cy, &DASH_TRAIL_CONFIG);
        }

        // Dust puff on dash start
        if self.player.is_dashing() && !was_dashing {
            let (cx, cy) = self.player.center();
            self.particles.burst(cx, cy + 4.0, &DUST_PUFF_CONFIG);
        }

        // Reset hit tracking when player starts a new attack
        let is_attacking = matches!(self.player.state, player::PlayerState::Attacking);
        if is_attacking && !was_attacking {
            for enemy in &mut self.enemies {
                enemy.hit_this_attack = false;
            }
        }

        // Combat: check player attack hitbox vs enemy hurtboxes
        if let Some(attack_hb) = self.player.attack_hitbox() {
            let (pcx, pcy) = self.player.center();
            for enemy in &mut self.enemies {
                if !enemy.alive || enemy.hit_this_attack {
                    continue;
                }
                let hurtbox = enemy.world_hurtbox();
                if attack_hb.overlaps(&hurtbox) {
                    let (ecx, ecy) = hurtbox.center();
                    let dx = ecx - pcx;
                    let dy = ecy - pcy;
                    let len = (dx * dx + dy * dy).sqrt().max(0.01);
                    let kb_dir_x = dx / len;
                    let kb_dir_y = dy / len;

                    let was_alive = enemy.alive;
                    enemy.take_damage(1, kb_dir_x, kb_dir_y);
                    enemy.hit_this_attack = true;

                    // Hit particles
                    self.particles.burst(ecx, ecy, &HIT_SPARK_CONFIG);

                    if !enemy.alive && was_alive {
                        // Kill: bigger hit pause, bigger shake, death burst
                        self.hit_pause_frames = 5;
                        self.camera.shake(5.0);
                        self.particles.burst(ecx, ecy, &DEATH_BURST_CONFIG);
                        self.damage_numbers.push(hud::DamageNumber::new(
                            1,
                            ecx - 2.0,
                            ecy - 8.0,
                            [255, 80, 80],
                        ));
                    } else {
                        // Hit: small hit pause, small shake
                        self.hit_pause_frames = 3;
                        self.camera.shake(2.5);
                        self.damage_numbers.push(hud::DamageNumber::new(
                            1,
                            ecx - 2.0,
                            ecy - 8.0,
                            [255, 255, 100],
                        ));
                    }
                }
            }
        }

        // Update all enemies
        let (pcx, pcy) = self.player.center();
        for enemy in &mut self.enemies {
            enemy.update(dt, &self.tilemap, pcx, pcy);
        }

        // Spawn projectiles from ghost enemies that fired
        for enemy in &self.enemies {
            if enemy.fired_projectile {
                let (ex, ey) = enemy.center();
                self.projectiles
                    .spawn(ex - 1.5, ey - 1.5, enemy.aim_dir_x, enemy.aim_dir_y);
            }
        }

        // Update projectiles
        let (trail_pos, impact_pos) = self.projectiles.update(dt_f32, &self.tilemap);
        for (tx, ty) in trail_pos {
            self.particles.burst(tx, ty, &PROJ_TRAIL_CONFIG);
        }
        for (ix, iy) in impact_pos {
            self.particles.burst(ix, iy, &PROJ_IMPACT_CONFIG);
        }

        // --- Enemy damage to player ---
        let player_hurtbox = self.player.world_hurtbox();
        let (pcx, pcy) = self.player.center();

        // Skeleton melee attacks
        for enemy in &self.enemies {
            if !enemy.alive || enemy.enemy_type != EnemyType::Skeleton {
                continue;
            }
            if let Some(atk_hb) = enemy.attack_hitbox() {
                if atk_hb.overlaps(&player_hurtbox) {
                    let (ecx, ecy) = enemy.center();
                    let dx = pcx - ecx;
                    let dy = pcy - ecy;
                    let len = (dx * dx + dy * dy).sqrt().max(0.01);
                    let died = self.player.take_damage(1, dx / len, dy / len);
                    if died {
                        self.hit_pause_frames = 8;
                        self.camera.shake(8.0);
                        self.particles.burst(pcx, pcy, &PLAYER_DEATH_BURST_CONFIG);
                        self.death_phase = DeathPhase::Dying;
                        self.death_timer = 0.0;
                    } else if self.player.invincible_timer > 0.0 {
                        self.hit_pause_frames = 4;
                        self.camera.shake(4.0);
                        self.particles.burst(pcx, pcy, &BLOOD_BURST_CONFIG);
                        self.damage_numbers.push(hud::DamageNumber::new(
                            1, pcx - 2.0, pcy - 8.0, [255, 80, 80],
                        ));
                    }
                    break; // only take one hit per frame
                }
            }
        }

        // Projectile hits
        if !self.player.is_dead() {
            let proj_hits = self.projectiles.check_player_hits(&self.player.world_hurtbox());
            for (hx, hy, dmg) in proj_hits {
                let dx = pcx - hx;
                let dy = pcy - hy;
                let len = (dx * dx + dy * dy).sqrt().max(0.01);
                let died = self.player.take_damage(dmg, dx / len, dy / len);
                if died {
                    self.hit_pause_frames = 8;
                    self.camera.shake(8.0);
                    self.particles.burst(pcx, pcy, &PLAYER_DEATH_BURST_CONFIG);
                    self.death_phase = DeathPhase::Dying;
                    self.death_timer = 0.0;
                    break;
                } else if self.player.invincible_timer > 0.0 {
                    self.hit_pause_frames = 4;
                    self.camera.shake(4.0);
                    self.particles.burst(hx, hy, &BLOOD_BURST_CONFIG);
                    self.damage_numbers.push(hud::DamageNumber::new(
                        dmg, hx - 2.0, hy - 8.0, [255, 80, 80],
                    ));
                    break; // one hit per frame due to i-frames
                }
            }
        }

        // Update particles
        self.particles.update(dt_f32);

        // Update damage numbers
        for dn in &mut self.damage_numbers {
            dn.update(dt_f32);
        }
        self.damage_numbers.retain(|dn| dn.alive());

        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }

        // Camera follows player center
        let (cx, cy) = self.player.center();
        self.camera.follow(cx, cy);
        self.camera.update(dt);
        self.camera.clamp_to_bounds(
            self.tilemap.pixel_width() as f32,
            self.tilemap.pixel_height() as f32,
        );

        true
    }

    fn render(&mut self, fb: &mut FrameBuffer, info: &FrameInfo, alpha: f32) {
        let fw = fb.width();
        let fh = fb.height();

        self.camera.viewport_w = fw;
        self.camera.viewport_h = fh;

        let (cam_x, cam_y) = self.camera.offset();

        // --- Draw tile map ---
        render_tilemap(fb, &self.tilemap, tiles::tile_sprite, cam_x, cam_y);

        // --- Draw enemies ---
        for enemy in &self.enemies {
            enemy.render(fb, alpha, cam_x, cam_y);
        }

        // --- Draw player ---
        let player_visible = if self.player.is_dead() {
            !self.player.animation.is_finished()
        } else if self.player.invincible_timer > 0.0 && !self.player.is_dashing() {
            // I-frame flashing: alternate every ~2 frames (66ms)
            ((self.player.invincible_timer * 15.0) as u32).is_multiple_of(2)
        } else {
            true
        };

        if player_visible {
            if self.flash_timer > 0 {
                self.player
                    .render_tinted(fb, alpha, cam_x, cam_y, ATTACK_TINT);
            } else if self.player.is_dashing() {
                self.player
                    .render_tinted(fb, alpha, cam_x, cam_y, DASH_TINT);
            } else if self.player.invincible_timer > 0.0 && !self.player.is_dead() {
                // I-frame white flash on visible frames
                if ((self.player.invincible_timer * 30.0) as u32).is_multiple_of(4) {
                    self.player
                        .render_tinted(fb, alpha, cam_x, cam_y, IFRAME_TINT);
                } else {
                    self.player.render(fb, alpha, cam_x, cam_y);
                }
            } else {
                self.player.render(fb, alpha, cam_x, cam_y);
            }
        }

        // --- Draw projectiles ---
        self.projectiles.render(fb, cam_x, cam_y);

        // --- Draw particles ---
        self.particles.render(fb, cam_x, cam_y);

        // --- Draw damage numbers ---
        for dn in &self.damage_numbers {
            dn.render(fb, cam_x, cam_y);
        }

        // --- Debug hitbox overlay ---
        if self.debug_hitboxes {
            self.render_debug_hitboxes(fb, cam_x, cam_y);
        }

        // --- Death fade overlay ---
        match self.death_phase {
            DeathPhase::FadeOut => {
                let opacity = (self.death_timer / DEATH_FADE_DURATION).min(1.0);
                fb.overlay([0, 0, 0], opacity);
            }
            DeathPhase::Dead => {
                fb.overlay([0, 0, 0], 1.0);
                // "YOU DIED" centered on screen
                let text = "YOU DIED";
                let tw = sprites::font::text_width(text);
                let tx = (fw as i32 - tw) / 2;
                let ty = (fh as i32) / 2 - 4;
                sprites::font::render_text(fb, text, tx, ty, [200, 30, 30]);

                // "PRESS Z TO RESTART" below
                let text2 = "PRESS ATTACK";
                let tw2 = sprites::font::text_width(text2);
                let tx2 = (fw as i32 - tw2) / 2;
                sprites::font::render_text(fb, text2, tx2, ty + 8, [150, 150, 150]);
                return;
            }
            _ => {}
        }

        // --- HUD ---
        let bar_h = 8;
        for y in 0..bar_h.min(fh) {
            for x in 0..fw {
                fb.set_pixel(x, y, [0, 0, 0]);
            }
        }

        // Health hearts at top-left
        hud::render_hearts(fb, self.player.hp, self.player.max_hp, 2, 1);

        // Performance bars (right-aligned, smaller)
        let bar_w = fw / 3;
        let bar_x = fw - bar_w;

        let fps_pixels = ((info.fps as usize) * bar_w) / 60;
        for x in bar_x..bar_x + fps_pixels.min(bar_w) {
            fb.set_pixel(x, 0, color::GREEN);
        }

        if info.cells_total > 0 {
            let ratio_pixels = (info.cells_redrawn * bar_w) / info.cells_total.max(1);
            for x in bar_x..bar_x + ratio_pixels.min(bar_w) {
                fb.set_pixel(x, 1, [255, 255, 0]);
            }
        }

        let frame_budget_us: u64 = 33_000;
        let draw_timing_bar = |fb: &mut FrameBuffer, y: usize, us: u64, c: Color| {
            let pixels = ((us as usize * bar_w) / frame_budget_us as usize).min(bar_w);
            for x in bar_x..bar_x + pixels {
                fb.set_pixel(x, y, c);
            }
        };
        draw_timing_bar(fb, 2, info.input_us, [0, 255, 255]);
        draw_timing_bar(fb, 3, info.render_us, [255, 80, 80]);
    }
}

impl CryptfallGame {
    fn restart(&mut self) {
        self.player = Player::new(120.0, 88.0);
        self.enemies = vec![
            Enemy::new_skeleton(160.0, 120.0, 11111),
            Enemy::new_skeleton(120.0, 150.0, 22222),
            Enemy::new_skeleton(60.0, 100.0, 33333),
            Enemy::new_ghost(180.0, 80.0, 44444),
            Enemy::new_ghost(100.0, 60.0, 55555),
        ];
        self.projectiles.clear();
        self.particles.clear();
        self.damage_numbers.clear();
        self.flash_timer = 0;
        self.hit_pause_frames = 0;
        self.death_phase = DeathPhase::Alive;
        self.death_timer = 0.0;
        self.idle_timer = 0.0;
        self.demo = None;

        let (cx, cy) = self.player.center();
        self.camera.follow(cx, cy);
        self.camera.snap();
    }

    fn render_debug_hitboxes(&self, fb: &mut FrameBuffer, cam_x: i32, cam_y: i32) {
        let phb = self.player.world_hurtbox();
        draw_aabb_outline(fb, &phb, cam_x, cam_y, color::GREEN);

        if let Some(ahb) = self.player.attack_hitbox() {
            draw_aabb_outline(fb, &ahb, cam_x, cam_y, color::RED);
        }

        for enemy in &self.enemies {
            if enemy.alive {
                let ehb = enemy.world_hurtbox();
                draw_aabb_outline(fb, &ehb, cam_x, cam_y, [0, 200, 0]);
            }
        }
    }
}

fn draw_aabb_outline(
    fb: &mut FrameBuffer,
    aabb: &engine::AABB,
    cam_x: i32,
    cam_y: i32,
    color: Color,
) {
    let x0 = aabb.x as i32 - cam_x;
    let y0 = aabb.y as i32 - cam_y;
    let x1 = (aabb.x + aabb.w) as i32 - cam_x;
    let y1 = (aabb.y + aabb.h) as i32 - cam_y;

    for x in x0..=x1 {
        fb.set_pixel_safe(x, y0, color);
        fb.set_pixel_safe(x, y1, color);
    }
    for y in y0..=y1 {
        fb.set_pixel_safe(x0, y, color);
        fb.set_pixel_safe(x1, y, color);
    }
}

#[rustfmt::skip]
fn create_test_room() -> TileMap {
    let layout = [
        "WWWWWWWWWWWWWWWWWWWWWWWWWWWWWW",
        "W............................W",
        "W............................W",
        "W....WWWW......WWWW..........W",
        "W....W..........W............W",
        "W....W..........W............W",
        "W................W...........W",
        "W............................W",
        "W............................W",
        "W........WW..................W",
        "W........WW..................W",
        "W............................W",
        "W............................W",
        "W...........WWWWWW...........W",
        "W............................W",
        "W............................W",
        "W..WW....................WW..W",
        "W..WW....................WW..W",
        "W............................W",
        "W............................W",
        "W............................W",
        "W............................W",
        "W............................W",
        "W............................W",
        "WWWWWWWWWWWWWWWWWWWWWWWWWWWWWW",
    ];
    let height = layout.len();
    let width = layout[0].len();
    let mut map = TileMap::new(width, height);

    for (y, row) in layout.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == 'W' {
                map.set(x, y, TileType::Wall);
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if map.get(x, y) == TileType::Wall && y + 1 < height && !map.is_solid(x, y + 1) {
                map.set(x, y, TileType::WallTop);
            }
        }
    }

    map
}

fn main() -> std::io::Result<()> {
    let mut terminal = engine::Terminal::new()?;
    let mut game = CryptfallGame::new();
    engine::run(&mut terminal, &mut game);
    Ok(())
}
