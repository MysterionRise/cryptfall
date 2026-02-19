mod enemy;
mod player;
mod sprites;
mod tiles;

use enemy::Enemy;
use engine::{
    color, render_tilemap, Camera, Color, FrameBuffer, FrameInfo, Game, GameKey, InputState,
    TileMap, TileType,
};
use player::Player;

const FLASH_FRAMES: u32 = 5;
const DEMO_IDLE_THRESHOLD: f32 = 5.0;

/// Dash i-frames tint: cool blue
const DASH_TINT: Color = [100, 160, 255];
/// Attack hit flash tint: warm red
const ATTACK_TINT: Color = [255, 80, 80];

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
            // Next attack in 1.5–4.5s
            self.attack_timer = 1.5 + (self.next_random() % 30) as f32 * 0.1;
            true
        } else {
            false
        };

        if self.timer <= 0.0 {
            // Change direction every 0.8–2.8s
            self.timer = 0.8 + (self.next_random() % 20) as f32 * 0.1;
            let dir = self.next_random() % 9; // 0-7 = directions, 8 = stop briefly
            let (dx, dy) = match dir {
                0 => (1.0, 0.0),
                1 => (-1.0, 0.0),
                2 => (0.0, 1.0),
                3 => (0.0, -1.0),
                4 => (FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                5 => (-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                6 => (FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                7 => (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                _ => (0.0, 0.0), // brief pause
            };
            self.dx = dx;
            self.dy = dy;
        }

        // Occasionally dash (1 in 30 chance per direction change)
        let dash = self.timer > 0.0
            && self.timer < dt
            && (self.dx != 0.0 || self.dy != 0.0)
            && self.next_random().is_multiple_of(5);

        (self.dx, self.dy, attack, dash)
    }
}

const FRAC_1_SQRT_2: f32 = std::f32::consts::FRAC_1_SQRT_2;

struct CryptfallGame {
    player: Player,
    enemies: Vec<Enemy>,
    tilemap: TileMap,
    camera: Camera,
    flash_timer: u32,
    idle_timer: f32,
    demo: Option<DemoState>,
}

impl CryptfallGame {
    fn new() -> Self {
        let tilemap = create_test_room();
        // Spawn player near center of 30×25 room (on open floor, row 12)
        let player = Player::new(120.0, 88.0);

        let mut camera = Camera::new(80, 48);
        let (cx, cy) = player.center();
        camera.follow(cx, cy);
        camera.snap();
        camera.clamp_to_bounds(tilemap.pixel_width() as f32, tilemap.pixel_height() as f32);

        let enemies = vec![
            Enemy::new(80.0, 60.0),
            Enemy::new(160.0, 120.0),
            Enemy::new(120.0, 150.0),
        ];

        Self {
            player,
            enemies,
            tilemap,
            camera,
            flash_timer: 0,
            idle_timer: 0.0,
            demo: None,
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

        let dt_f32 = dt as f32;

        // Demo mode management
        if has_input(input) {
            self.idle_timer = 0.0;
            self.demo = None;
        } else {
            self.idle_timer += dt_f32;
        }

        let was_attacking = matches!(self.player.state, player::PlayerState::Attacking);

        if self.demo.is_some() || self.idle_timer >= DEMO_IDLE_THRESHOLD {
            // Enter or continue demo mode
            let demo = self.demo.get_or_insert_with(DemoState::new);
            let (dx, dy, attack, dash) = demo.update(dt_f32);
            self.player
                .update_with_input(dx, dy, attack, dash, dt, &self.tilemap);

            // Trigger flash on demo attacks
            if attack && self.player.attack_cooldown > 0.0 {
                self.flash_timer = FLASH_FRAMES;
                self.camera.shake(3.0);
            }
            if dash {
                self.camera.shake(6.0);
            }
        } else {
            // Normal play
            self.player.update(input, dt, &self.tilemap);

            // Attack: trigger red flash + small screen shake (only if cooldown allows)
            if input.is_pressed(GameKey::Attack) && self.player.attack_cooldown > 0.0 {
                self.flash_timer = FLASH_FRAMES;
                self.camera.shake(3.0);
            }

            // Dash: trigger bigger screen shake
            if input.is_pressed(GameKey::Dash) {
                self.camera.shake(6.0);
            }
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
                    // Calculate knockback direction: player center → enemy center
                    let (ecx, ecy) = hurtbox.center();
                    let dx = ecx - pcx;
                    let dy = ecy - pcy;
                    let len = (dx * dx + dy * dy).sqrt().max(0.01);
                    let kb_strength = 4.0;
                    let kb_dx = dx / len * kb_strength;
                    let kb_dy = dy / len * kb_strength;

                    enemy.take_damage(1, kb_dx, kb_dy, &self.tilemap);
                    enemy.hit_this_attack = true;
                    self.camera.shake(2.5);
                }
            }
        }

        // Update all enemies
        for enemy in &mut self.enemies {
            enemy.update(dt);
        }

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

        // Update camera viewport to match current framebuffer
        self.camera.viewport_w = fw;
        self.camera.viewport_h = fh;

        let (cam_x, cam_y) = self.camera.offset();

        // --- Draw tile map ---
        render_tilemap(fb, &self.tilemap, tiles::tile_sprite, cam_x, cam_y);

        // --- Draw enemies (between tilemap and player) ---
        for enemy in &self.enemies {
            enemy.render(fb, alpha, cam_x, cam_y);
        }

        // --- Draw player ---
        // Determine tint: attack flash (red) takes priority over dash i-frames (blue)
        if self.flash_timer > 0 {
            self.player
                .render_tinted(fb, alpha, cam_x, cam_y, ATTACK_TINT);
        } else if self.player.is_dashing() {
            self.player
                .render_tinted(fb, alpha, cam_x, cam_y, DASH_TINT);
        } else {
            self.player.render(fb, alpha, cam_x, cam_y);
        }

        // --- HUD (not affected by camera/shake) ---
        let bar_h = 4;
        for y in 0..bar_h.min(fh) {
            for x in 0..fw {
                fb.set_pixel(x, y, [0, 0, 0]);
            }
        }

        // FPS bar (green)
        let fps_pixels = (info.fps as usize).min(fw);
        for x in 0..fps_pixels {
            fb.set_pixel(x, 0, color::GREEN);
        }

        // Cells redrawn bar (yellow)
        if info.cells_total > 0 {
            let ratio_pixels = (info.cells_redrawn * fw) / info.cells_total.max(1);
            for x in 0..ratio_pixels.min(fw) {
                fb.set_pixel(x, 1, [255, 255, 0]);
            }
        }

        // Timing bars
        let frame_budget_us: u64 = 33_000;
        let draw_timing_bar = |fb: &mut FrameBuffer, y: usize, us: u64, c: Color| {
            let pixels = ((us as usize * fw) / frame_budget_us as usize).min(fw);
            for x in 0..pixels {
                fb.set_pixel(x, y, c);
            }
        };
        draw_timing_bar(fb, 2, info.input_us, [0, 255, 255]);
        draw_timing_bar(fb, 3, info.render_us, [255, 80, 80]);
    }
}

/// Create a 30×25 test room with interior walls and pillars.
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

    // Post-process: Wall tiles with floor below become WallTop (visible ledge)
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
