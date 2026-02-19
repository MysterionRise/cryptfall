mod player;
mod sprites;
mod tiles;

use engine::{
    color, render_tilemap, Camera, Color, FrameBuffer, FrameInfo, Game, GameKey, InputState,
    TileMap, TileType,
};
use player::Player;

const FLASH_FRAMES: u32 = 5;

struct CryptfallGame {
    player: Player,
    tilemap: TileMap,
    camera: Camera,
    flash_timer: u32,
}

impl CryptfallGame {
    fn new() -> Self {
        let tilemap = create_test_room();
        // Spawn player near center of 30×25 room (pixel 116, 94)
        let player = Player::new(116.0, 94.0);

        let mut camera = Camera::new(80, 48);
        let (cx, cy) = player.center();
        camera.follow(cx, cy);
        camera.snap();
        camera.clamp_to_bounds(tilemap.pixel_width() as f32, tilemap.pixel_height() as f32);

        Self {
            player,
            tilemap,
            camera,
            flash_timer: 0,
        }
    }
}

impl Game for CryptfallGame {
    fn update(&mut self, input: &InputState, dt: f64) -> bool {
        if input.is_pressed(GameKey::Quit) {
            return false;
        }

        self.player.update(input, dt, &self.tilemap);

        // Camera follows player center
        let (cx, cy) = self.player.center();
        self.camera.follow(cx, cy);
        self.camera.update(dt);
        self.camera.clamp_to_bounds(
            self.tilemap.pixel_width() as f32,
            self.tilemap.pixel_height() as f32,
        );

        // Attack: trigger red flash + small screen shake
        if input.is_pressed(GameKey::Attack) {
            self.flash_timer = FLASH_FRAMES;
            self.camera.shake(3.0);
        }
        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }

        // Dash: trigger bigger screen shake
        if input.is_pressed(GameKey::Dash) {
            self.camera.shake(6.0);
        }

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

        // --- Draw player ---
        if self.flash_timer > 0 {
            self.player
                .render_tinted(fb, alpha, cam_x, cam_y, [255, 80, 80]);
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
