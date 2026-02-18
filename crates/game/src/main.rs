use std::collections::VecDeque;

use engine::{color, Color, FrameBuffer, FrameInfo, Game, GameKey, InputState, Transform, Vec2};

const SQ_SIZE: f32 = 6.0;
const MOVE_SPEED: f32 = 60.0; // pixels per second
const DASH_DISTANCE: f32 = 20.0;
const TRAIL_LENGTH: usize = 5;

/// Per-key visual state for the HUD.
#[derive(Clone, Copy)]
enum KeyVisual {
    Inactive,
    Pressed,
    Held,
    Released,
}

struct CryptfallGame {
    transform: Transform,
    velocity: Vec2,
    trail: VecDeque<Vec2>,
    key_visuals: [(GameKey, KeyVisual); 7],
}

impl CryptfallGame {
    fn new() -> Self {
        Self {
            transform: Transform::new(40.0, 30.0),
            velocity: Vec2::ZERO,
            trail: VecDeque::with_capacity(TRAIL_LENGTH + 1),
            key_visuals: [
                (GameKey::Up, KeyVisual::Inactive),
                (GameKey::Down, KeyVisual::Inactive),
                (GameKey::Left, KeyVisual::Inactive),
                (GameKey::Right, KeyVisual::Inactive),
                (GameKey::Attack, KeyVisual::Inactive),
                (GameKey::Dash, KeyVisual::Inactive),
                (GameKey::Pause, KeyVisual::Inactive),
            ],
        }
    }
}

impl Game for CryptfallGame {
    fn update(&mut self, input: &InputState, dt: f64) -> bool {
        if input.is_pressed(GameKey::Quit) {
            return false;
        }

        // Snapshot key visuals for rendering
        for entry in &mut self.key_visuals {
            entry.1 = if input.is_pressed(entry.0) {
                KeyVisual::Pressed
            } else if input.is_held(entry.0) {
                KeyVisual::Held
            } else if input.is_released(entry.0) {
                KeyVisual::Released
            } else {
                KeyVisual::Inactive
            };
        }

        // Save previous position for interpolation
        self.transform.commit();

        // Record trail position before moving
        self.trail.push_back(self.transform.position);
        if self.trail.len() > TRAIL_LENGTH {
            self.trail.pop_front();
        }

        let (dx, dy) = input.direction();
        self.velocity = Vec2::new(dx * MOVE_SPEED, dy * MOVE_SPEED);

        // Dash: instant jump in current direction
        if input.is_pressed(GameKey::Dash) && (dx != 0.0 || dy != 0.0) {
            self.transform.position.x += dx * DASH_DISTANCE;
            self.transform.position.y += dy * DASH_DISTANCE;
        }

        // Apply velocity
        let dt = dt as f32;
        self.transform.position.x += self.velocity.x * dt;
        self.transform.position.y += self.velocity.y * dt;

        true
    }

    fn render(&mut self, fb: &mut FrameBuffer, info: &FrameInfo, alpha: f32) {
        let fw = fb.width();
        let fh = fb.height();
        let w = fw as f32;
        let h = fh as f32;

        // Clamp position to bounds
        let pos = self.transform.interpolated(alpha);
        let px = pos.x.clamp(0.0, (w - SQ_SIZE).max(0.0));
        let py = pos.y.clamp(0.0, (h - SQ_SIZE).max(0.0));

        // Clamp actual position so it doesn't drift off screen
        self.transform.position.x = self.transform.position.x.clamp(0.0, (w - SQ_SIZE).max(0.0));
        self.transform.position.y = self.transform.position.y.clamp(0.0, (h - SQ_SIZE).max(0.0));

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

        // --- Draw trail ---
        let trail_len = self.trail.len();
        for (i, trail_pos) in self.trail.iter().enumerate() {
            let brightness = ((i + 1) as f32 / trail_len as f32 * 0.7 * 255.0) as u8;
            let trail_color: Color = [brightness, brightness, brightness];
            let tx = trail_pos.x.clamp(0.0, (w - SQ_SIZE).max(0.0)) as usize;
            let ty = trail_pos.y.clamp(0.0, (h - SQ_SIZE).max(0.0)) as usize;
            fb.fill_rect(tx, ty, SQ_SIZE as usize, SQ_SIZE as usize, trail_color);
        }

        // --- Draw player square ---
        fb.fill_rect(
            px as usize,
            py as usize,
            SQ_SIZE as usize,
            SQ_SIZE as usize,
            color::WHITE,
        );

        // --- HUD: top bar (6 pixel rows = 3 terminal rows) ---
        let bar_h = 6;
        for y in 0..bar_h.min(fh) {
            for x in 0..fw {
                fb.set_pixel(x, y, [0, 0, 0]);
            }
        }

        // Row 0: green bar = FPS (1 pixel per FPS unit)
        let fps_pixels = (info.fps as usize).min(fw);
        for x in 0..fps_pixels {
            fb.set_pixel(x, 0, color::GREEN);
        }

        // Row 1: yellow bar = cells redrawn ratio
        if info.cells_total > 0 {
            let ratio_pixels = (info.cells_redrawn * fw) / info.cells_total.max(1);
            for x in 0..ratio_pixels.min(fw) {
                fb.set_pixel(x, 1, [255, 255, 0]);
            }
        }

        // Row 2-3: timing bars (input=cyan, update=magenta, render=red)
        // Scale: full width = 33ms (one full frame budget)
        let frame_budget_us: u64 = 33_000;
        let draw_timing_bar = |fb: &mut FrameBuffer, y: usize, us: u64, c: Color| {
            let pixels = ((us as usize * fw) / frame_budget_us as usize).min(fw);
            for x in 0..pixels {
                fb.set_pixel(x, y, c);
            }
        };
        draw_timing_bar(fb, 2, info.input_us, [0, 255, 255]); // cyan
        draw_timing_bar(fb, 3, info.update_us, [255, 0, 255]); // magenta
        draw_timing_bar(fb, 4, info.render_us, [255, 80, 80]); // red

        // Row 4-5: input debug indicators
        let x_offsets = [0, 2, 4, 6, 9, 11, 14];
        let held_colors: [Color; 7] = [
            color::BLUE,
            color::BLUE,
            color::BLUE,
            color::BLUE,
            color::RED,
            [255, 0, 255],
            [255, 128, 0],
        ];
        for (i, &(_, visual)) in self.key_visuals.iter().enumerate() {
            let x_off = x_offsets[i];
            if x_off + 1 < fw && 5 < fh {
                let c = match visual {
                    KeyVisual::Pressed => color::WHITE,
                    KeyVisual::Held => held_colors[i],
                    KeyVisual::Released => [80, 80, 80],
                    KeyVisual::Inactive => [30, 30, 30],
                };
                fb.set_pixel(x_off, 4, c);
                fb.set_pixel(x_off + 1, 4, c);
                fb.set_pixel(x_off, 5, c);
                fb.set_pixel(x_off + 1, 5, c);
            }
        }

        // --- Bottom bar: "Press Q to quit" indicator ---
        // Draw a row of colored pixels at the bottom as a quit hint
        if fh >= 2 {
            let bottom_y = fh - 2;
            // Dark bar
            for x in 0..fw {
                fb.set_pixel(x, bottom_y, [0, 0, 0]);
                fb.set_pixel(x, bottom_y + 1, [0, 0, 0]);
            }
            // Red "Q" indicator: 3x2 red block at left
            for dy in 0..2 {
                for dx in 0..3_usize {
                    if dx < fw {
                        fb.set_pixel(dx, bottom_y + dy, color::RED);
                    }
                }
            }
        }

    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = engine::Terminal::new()?;
    let mut game = CryptfallGame::new();
    engine::run(&mut terminal, &mut game);
    Ok(())
}
