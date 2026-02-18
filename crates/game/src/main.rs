use engine::{color, GameKey, InputState};

const SQ_SIZE: f32 = 6.0;
const MOVE_SPEED: f32 = 2.0; // pixels per frame
const DASH_DISTANCE: f32 = 20.0;

fn main() -> std::io::Result<()> {
    let mut terminal = engine::Terminal::new()?;

    let mut sq_x: f32 = 10.0;
    let mut sq_y: f32 = 10.0;

    engine::run(&mut terminal, |fb, input: &InputState, info| {
        let w = fb.width() as f32;
        let h = fb.height() as f32;

        // Quit
        if input.is_pressed(GameKey::Quit) {
            return false;
        }

        // Movement
        let (dx, dy) = input.direction();

        // Dash: jump 20 pixels in current direction
        if input.is_pressed(GameKey::Dash) && (dx != 0.0 || dy != 0.0) {
            sq_x += dx * DASH_DISTANCE;
            sq_y += dy * DASH_DISTANCE;
        }

        // Smooth movement while held
        sq_x += dx * MOVE_SPEED;
        sq_y += dy * MOVE_SPEED;

        // Clamp to bounds
        sq_x = sq_x.clamp(0.0, (w - SQ_SIZE).max(0.0));
        sq_y = sq_y.clamp(0.0, (h - SQ_SIZE).max(0.0));

        // Draw RGB gradient background
        let fw = fb.width();
        let fh = fb.height();
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
                let b = 80;
                fb.set_pixel(x, y, [r, g, b]);
            }
        }

        // Draw white square
        fb.fill_rect(
            sq_x as usize,
            sq_y as usize,
            SQ_SIZE as usize,
            SQ_SIZE as usize,
            color::WHITE,
        );

        // HUD: dark bar at top (4 pixel rows = 2 terminal rows)
        let bar_h = 4;
        for y in 0..bar_h.min(fh) {
            for x in 0..fw {
                fb.set_pixel(x, y, [0, 0, 0]);
            }
        }

        // Row 0: green bar = FPS
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

        // Row 2: input debug — light up pixels for held keys
        // Layout: [Up][Down][Left][Right] [Attack][Dash] [Pause]
        let key_indicators: &[(GameKey, usize, engine::Color)] = &[
            (GameKey::Up, 0, color::BLUE),
            (GameKey::Down, 2, color::BLUE),
            (GameKey::Left, 4, color::BLUE),
            (GameKey::Right, 6, color::BLUE),
            (GameKey::Attack, 9, color::RED),
            (GameKey::Dash, 11, [255, 0, 255]),
            (GameKey::Pause, 14, [255, 128, 0]),
        ];
        for &(key, x_off, held_color) in key_indicators {
            if x_off + 1 < fw {
                let c = if input.is_pressed(key) {
                    color::WHITE // bright flash on press
                } else if input.is_held(key) {
                    held_color
                } else if input.is_released(key) {
                    [80, 80, 80] // dim on release
                } else {
                    [30, 30, 30] // dark = inactive
                };
                fb.set_pixel(x_off, 2, c);
                fb.set_pixel(x_off + 1, 2, c);
                fb.set_pixel(x_off, 3, c);
                fb.set_pixel(x_off + 1, 3, c);
            }
        }

        true
    });

    Ok(())
}
