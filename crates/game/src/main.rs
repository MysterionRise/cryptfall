mod player;
mod sprites;
mod tiles;

use engine::{color, render_tilemap, Color, FrameBuffer, FrameInfo, Game, GameKey, InputState};
use engine::{TileMap, TileType};
use player::Player;

const FLASH_FRAMES: u32 = 5;

struct CryptfallGame {
    player: Player,
    tilemap: TileMap,
    flash_timer: u32,
}

impl CryptfallGame {
    fn new() -> Self {
        let tilemap = create_test_room();
        // Spawn player near center of the room (tile 7,5 → pixel 56,40)
        let player = Player::new(56.0, 40.0);
        Self {
            player,
            tilemap,
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

        // Attack: trigger red flash
        if input.is_pressed(GameKey::Attack) {
            self.flash_timer = FLASH_FRAMES;
        }
        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }

        true
    }

    fn render(&mut self, fb: &mut FrameBuffer, info: &FrameInfo, alpha: f32) {
        let fw = fb.width();
        let fh = fb.height();

        // Camera is static for now
        let cam_x = 0;
        let cam_y = 0;

        // --- Draw tile map ---
        render_tilemap(fb, &self.tilemap, tiles::tile_sprite, cam_x, cam_y);

        // --- Draw player ---
        if self.flash_timer > 0 {
            self.player
                .render_tinted(fb, alpha, cam_x, cam_y, [255, 80, 80]);
        } else {
            self.player.render(fb, alpha, cam_x, cam_y);
        }

        // --- HUD ---
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

/// Create the 15×12 test room.
fn create_test_room() -> TileMap {
    let layout = [
        "WWWWWWWWWWWWWWW",
        "W.............W",
        "W.............W",
        "W.............W",
        "W.............W",
        "W......W......W",
        "W......W......W",
        "W.............W",
        "W.............W",
        "W.............W",
        "W.............W",
        "WWWWWWWWWWWWWWW",
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
