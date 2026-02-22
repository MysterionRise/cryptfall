mod combat;
mod enemies;
mod hud;
mod player;
mod projectile;
mod sprites;
mod tiles;
mod tuning;
mod waves;

use engine::{
    color, render_tilemap, BurstConfig, Camera, Color, FrameBuffer, FrameInfo, Game, GameKey,
    InputState, ParticleSystem, TileMap, TileType,
};
use player::Player;
use tuning::*;
use waves::WaveManager;

// --- Particle burst configurations for player movement effects ---

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

const FRAC_1_SQRT_2: f32 = std::f32::consts::FRAC_1_SQRT_2;

// --- Demo mode auto-play ---

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

// --- Death sequence phases ---

enum DeathPhase {
    Alive,
    Dying,
    FadeOut,
    Dead,
}

// --- Main game state ---

struct CryptfallGame {
    player: Player,
    enemies: Vec<enemies::Enemy>,
    projectiles: projectile::ProjectileSystem,
    tilemap: TileMap,
    camera: Camera,
    particles: ParticleSystem,
    damage_numbers: Vec<hud::DamageNumber>,
    waves: WaveManager,
    flash_timer: u32,
    hit_pause_frames: u32,
    idle_timer: f32,
    demo: Option<DemoState>,
    debug_hitboxes: bool,
    death_phase: DeathPhase,
    death_timer: f32,
    heart_flash_timer: f32,
    last_hp: i32,
}

impl CryptfallGame {
    fn new() -> Self {
        let tilemap = create_arena();
        let player = Player::new(75.0, 53.0);

        let mut camera = Camera::new(80, 48);
        let (cx, cy) = player.center();
        camera.follow(cx, cy);
        camera.snap();
        camera.clamp_to_bounds(tilemap.pixel_width() as f32, tilemap.pixel_height() as f32);

        let waves = WaveManager::new();
        let enemies = waves.spawn_initial();

        Self {
            player,
            enemies,
            projectiles: projectile::ProjectileSystem::new(),
            tilemap,
            camera,
            particles: ParticleSystem::new(),
            damage_numbers: Vec::new(),
            waves,
            flash_timer: 0,
            hit_pause_frames: 0,
            idle_timer: 0.0,
            demo: None,
            debug_hitboxes: false,
            death_phase: DeathPhase::Alive,
            death_timer: 0.0,
            heart_flash_timer: 0.0,
            last_hp: 5,
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

        if input.is_pressed(GameKey::Pause) {
            self.debug_hitboxes = !self.debug_hitboxes;
        }

        let dt_f32 = dt as f32;

        // Hit pause: freeze all game logic
        if self.hit_pause_frames > 0 {
            self.hit_pause_frames -= 1;
            self.particles.update(dt_f32);
            return true;
        }

        // Death sequence management
        match self.death_phase {
            DeathPhase::Alive => {}
            DeathPhase::Dying => {
                self.death_timer += dt_f32;
                self.player
                    .update_with_input(0.0, 0.0, false, false, dt, &self.tilemap);
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
                if input.is_pressed(GameKey::Attack) {
                    self.restart();
                }
                return true;
            }
        }

        // Victory: wait for restart
        if self.waves.victory {
            if input.is_pressed(GameKey::Attack) {
                self.restart();
            }
            self.particles.update(dt_f32);
            return true;
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

        // --- Combat: player attacks enemies ---
        let effects = combat::check_player_attacks(
            &self.player,
            &mut self.enemies,
            &mut self.particles,
            &mut self.damage_numbers,
        );
        if effects.hit_pause_frames > 0 {
            self.hit_pause_frames = effects.hit_pause_frames;
        }
        if effects.camera_shake > 0.0 {
            self.camera.shake(effects.camera_shake);
        }

        // Update enemies
        let (pcx, pcy) = self.player.center();
        for enemy in &mut self.enemies {
            enemy.update(dt, &self.tilemap, pcx, pcy);
        }

        // Projectile spawning and physics
        combat::spawn_enemy_projectiles(&self.enemies, &mut self.projectiles);
        combat::update_projectiles(
            &mut self.projectiles,
            &self.tilemap,
            &mut self.particles,
            dt_f32,
        );

        // --- Combat: enemies attack player ---
        let effects = combat::check_enemy_attacks(
            &mut self.player,
            &self.enemies,
            &mut self.projectiles,
            &mut self.particles,
            &mut self.damage_numbers,
        );
        if effects.hit_pause_frames > 0 {
            self.hit_pause_frames = effects.hit_pause_frames;
        }
        if effects.camera_shake > 0.0 {
            self.camera.shake(effects.camera_shake);
        }
        if effects.player_died {
            self.death_phase = DeathPhase::Dying;
            self.death_timer = 0.0;
        }

        // Heart flash on HP loss
        if self.player.hp < self.last_hp {
            self.heart_flash_timer = 0.3;
        }
        self.last_hp = self.player.hp;
        if self.heart_flash_timer > 0.0 {
            self.heart_flash_timer -= dt_f32;
        }

        // Update particles and damage numbers
        self.particles.update(dt_f32);
        for dn in &mut self.damage_numbers {
            dn.update(dt_f32);
        }
        self.damage_numbers.retain(|dn| dn.alive());

        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }

        // Wave progression
        self.waves.update_announce(dt_f32);
        if let Some(new_enemies) = self.waves.check_progression(&self.enemies, dt_f32) {
            self.enemies = new_enemies;
            self.projectiles.clear();
        }
        self.waves.update_clear_display(dt_f32);

        // Camera follows player center
        let (cx, cy) = self.player.center();
        self.camera.follow(cx, cy);
        self.camera.update(dt);
        self.camera
            .clamp_to_bounds(self.tilemap.pixel_width() as f32, self.tilemap.pixel_height() as f32);

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
                let text = "YOU DIED";
                let tw = sprites::font::text_width(text);
                let tx = (fw as i32 - tw) / 2;
                let ty = (fh as i32) / 2 - 4;
                sprites::font::render_text(fb, text, tx, ty, [200, 30, 30]);

                let text2 = "PRESS ATTACK";
                let tw2 = sprites::font::text_width(text2);
                let tx2 = (fw as i32 - tw2) / 2;
                sprites::font::render_text(fb, text2, tx2, ty + 8, [150, 150, 150]);
                return;
            }
            _ => {}
        }

        // --- Victory overlay ---
        if self.waves.victory {
            fb.overlay([0, 0, 0], 0.5);
            let text = "VICTORY";
            let tw = sprites::font::text_width(text);
            let tx = (fw as i32 - tw) / 2;
            let ty = (fh as i32) / 2 - 4;
            sprites::font::render_text(fb, text, tx, ty, [255, 220, 50]);

            let text2 = "PRESS ATTACK";
            let tw2 = sprites::font::text_width(text2);
            let tx2 = (fw as i32 - tw2) / 2;
            sprites::font::render_text(fb, text2, tx2, ty + 8, [150, 150, 150]);
        }

        // --- Wave announce ---
        if self.waves.wave_announce_timer > 0.0 && !self.waves.victory {
            let wave_text = match self.waves.current_wave {
                1 => "WAVE 1",
                2 => "WAVE 2",
                3 => "WAVE 3",
                _ => "WAVE",
            };
            let tw = sprites::font::text_width(wave_text);
            let tx = (fw as i32 - tw) / 2;
            let ty = 10;
            let alpha_val =
                (self.waves.wave_announce_timer / waves::WAVE_ANNOUNCE_DURATION).min(1.0);
            let brightness = (255.0 * alpha_val) as u8;
            sprites::font::render_text(
                fb,
                wave_text,
                tx,
                ty,
                [brightness, brightness, brightness],
            );
        }

        // --- Wave clear display ---
        if self.waves.wave_clear_display_timer > 0.0 {
            let text = "WAVE CLEAR";
            let tw = sprites::font::text_width(text);
            let tx = (fw as i32 - tw) / 2;
            let ty = (fh as i32) / 2 - 4;
            let alpha_val = (self.waves.wave_clear_display_timer / 1.5).min(1.0);
            let g = (255.0 * alpha_val) as u8;
            sprites::font::render_text(fb, text, tx, ty, [g, 255, g]);
        }

        // --- HUD ---
        let bar_h = 8;
        for y in 0..bar_h.min(fh) {
            for x in 0..fw {
                fb.set_pixel(x, y, [0, 0, 0]);
            }
        }

        hud::render_hearts(fb, self.player.hp, self.player.max_hp, 2, 1);

        // Heart flash overlay on damage
        if self.heart_flash_timer > 0.0 {
            let flash_intensity = (self.heart_flash_timer / 0.3).min(1.0);
            let flash_color: Color = [
                (255.0 * flash_intensity) as u8,
                (255.0 * flash_intensity) as u8,
                (255.0 * flash_intensity) as u8,
            ];
            // Flash the HUD hearts region
            for y in 1..6 {
                for x in 2..(2 + self.player.max_hp as usize * 6) {
                    if let Some(c) = fb.get_pixel(x, y) {
                        let blended = [
                            ((c[0] as f32 + flash_color[0] as f32) / 2.0).min(255.0) as u8,
                            ((c[1] as f32 + flash_color[1] as f32) / 2.0).min(255.0) as u8,
                            ((c[2] as f32 + flash_color[2] as f32) / 2.0).min(255.0) as u8,
                        ];
                        fb.set_pixel(x, y, blended);
                    }
                }
            }
        }

        let wave_hud = match self.waves.current_wave {
            1 => "1",
            2 => "2",
            3 => "3",
            _ => "",
        };
        if !wave_hud.is_empty() {
            let whw = sprites::font::text_width(wave_hud);
            let whx = (fw as i32 - whw) / 2;
            sprites::font::render_text(fb, wave_hud, whx, 1, [180, 180, 180]);
        }

        // Performance bars
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
        self.player = Player::new(75.0, 53.0);
        self.waves.reset();
        self.enemies = self.waves.spawn_initial();
        self.projectiles.clear();
        self.particles.clear();
        self.damage_numbers.clear();
        self.flash_timer = 0;
        self.hit_pause_frames = 0;
        self.death_phase = DeathPhase::Alive;
        self.death_timer = 0.0;
        self.idle_timer = 0.0;
        self.demo = None;
        self.heart_flash_timer = 0.0;
        self.last_hp = 5;

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
fn create_arena() -> TileMap {
    let layout = [
        "WWWWWWWWWWWWWWWWWWWW",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "W..................W",
        "WWWWWWWWWWWWWWWWWWWW",
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
