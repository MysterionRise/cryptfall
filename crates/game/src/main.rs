mod player;
mod sprites;

use engine::{color, Color, FrameBuffer, FrameInfo, Game, GameKey, InputState};
use player::Player;

const FLASH_FRAMES: u32 = 5;

struct CryptfallGame {
    player: Player,
    flash_timer: u32,
}

impl CryptfallGame {
    fn new() -> Self {
        Self {
            player: Player::new(40.0, 30.0),
            flash_timer: 0,
        }
    }
}

impl Game for CryptfallGame {
    fn update(&mut self, input: &InputState, dt: f64) -> bool {
        if input.is_pressed(GameKey::Quit) {
            return false;
        }

        self.player.update(input, dt);

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

        // --- Draw gradient background ---
        for y in 0..fh {
            for x in 0..fw {
                let r = if fw > 1 {
                    (x * 255 / (fw - 1)) as u8
                } else {
                    0
                };
                let g = if fh > 1 {
                    (y * 255 / (fh - 1)) as u8
                } else {
                    0
                };
                fb.set_pixel(x, y, [r, g, 80]);
            }
        }

        // --- Draw player ---
        if self.flash_timer > 0 {
            self.player.render_tinted(fb, alpha, [255, 80, 80]);
        } else {
            self.player.render(fb, alpha);
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

fn main() -> std::io::Result<()> {
    let mut terminal = engine::Terminal::new()?;
    let mut game = CryptfallGame::new();
    engine::run(&mut terminal, &mut game);
    Ok(())
}
