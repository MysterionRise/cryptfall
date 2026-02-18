mod sprites;

use engine::{color, Color, FrameBuffer, FrameInfo, Game, GameKey, InputState, Transform};

const MOVE_SPEED: f32 = 60.0; // pixels per second
const DASH_DISTANCE: f32 = 20.0;
const FLASH_FRAMES: u32 = 5;

struct CryptfallGame {
    transform: Transform,
    facing_right: bool,
    flash_timer: u32, // frames remaining for red tint
}

impl CryptfallGame {
    fn new() -> Self {
        Self {
            transform: Transform::new(40.0, 30.0),
            facing_right: true,
            flash_timer: 0,
        }
    }
}

impl Game for CryptfallGame {
    fn update(&mut self, input: &InputState, dt: f64) -> bool {
        if input.is_pressed(GameKey::Quit) {
            return false;
        }

        self.transform.commit();

        let (dx, dy) = input.direction();

        // Update facing direction
        if dx > 0.0 {
            self.facing_right = true;
        } else if dx < 0.0 {
            self.facing_right = false;
        }

        // Dash
        if input.is_pressed(GameKey::Dash) && (dx != 0.0 || dy != 0.0) {
            self.transform.position.x += dx * DASH_DISTANCE;
            self.transform.position.y += dy * DASH_DISTANCE;
        }

        // Attack: trigger red flash
        if input.is_pressed(GameKey::Attack) {
            self.flash_timer = FLASH_FRAMES;
        }
        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }

        // Apply velocity
        let dt = dt as f32;
        self.transform.position.x += dx * MOVE_SPEED * dt;
        self.transform.position.y += dy * MOVE_SPEED * dt;

        true
    }

    fn render(&mut self, fb: &mut FrameBuffer, info: &FrameInfo, alpha: f32) {
        let fw = fb.width();
        let fh = fb.height();
        let w = fw as f32;
        let h = fh as f32;

        let sprite = &sprites::PLAYER_TEST;

        // Clamp position
        self.transform.position.x = self
            .transform
            .position
            .x
            .clamp(0.0, (w - sprite.width as f32).max(0.0));
        self.transform.position.y = self
            .transform
            .position
            .y
            .clamp(0.0, (h - sprite.height as f32).max(0.0));

        let pos = self.transform.interpolated(alpha);
        let px = pos.x.clamp(0.0, (w - sprite.width as f32).max(0.0)) as i32;
        let py = pos.y.clamp(0.0, (h - sprite.height as f32).max(0.0)) as i32;

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

        // --- Draw sprite ---
        let tinting = self.flash_timer > 0;
        let tint: Color = [255, 80, 80]; // red flash

        match (self.facing_right, tinting) {
            (true, false) => fb.blit_sprite(sprite, px, py),
            (false, false) => fb.blit_sprite_flipped(sprite, px, py),
            (true, true) => fb.blit_sprite_tinted(sprite, px, py, tint),
            (false, true) => fb.blit_sprite_flipped_tinted(sprite, px, py, tint),
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
