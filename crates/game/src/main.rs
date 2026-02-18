use crossterm::event::{Event, KeyCode, KeyEvent};
use engine::{color, FrameBuffer};

fn main() -> std::io::Result<()> {
    let mut terminal = engine::Terminal::new()?;

    let mut sq_x: usize = 10;
    let mut sq_y: usize = 10;
    const SQ_SIZE: usize = 6;

    engine::run(&mut terminal, |fb: &mut FrameBuffer, events, info| {
        let w = fb.width();
        let h = fb.height();

        // Handle input
        for evt in events {
            if let Event::Key(KeyEvent { code, .. }) = evt {
                match code {
                    KeyCode::Char('q') => return false,
                    KeyCode::Up => sq_y = sq_y.saturating_sub(1),
                    KeyCode::Down => sq_y = (sq_y + 1).min(h.saturating_sub(SQ_SIZE)),
                    KeyCode::Left => sq_x = sq_x.saturating_sub(1),
                    KeyCode::Right => sq_x = (sq_x + 1).min(w.saturating_sub(SQ_SIZE)),
                    _ => {}
                }
            }
        }

        // Draw RGB gradient background
        for y in 0..h {
            for x in 0..w {
                let r = if w > 1 { (x * 255 / (w - 1)) as u8 } else { 0 };
                let g = if h > 1 { (y * 255 / (h - 1)) as u8 } else { 0 };
                let b = 80;
                fb.set_pixel(x, y, [r, g, b]);
            }
        }

        // Draw white square
        fb.fill_rect(sq_x, sq_y, SQ_SIZE, SQ_SIZE, color::WHITE);

        // Draw stats as pixels (write text by coloring a bar at top)
        // We'll use a simple approach: dark bar at top for readability
        let bar_h = 2; // 2 pixel rows = 1 terminal row
        for y in 0..bar_h {
            for x in 0..w {
                fb.set_pixel(x, y, [0, 0, 0]);
            }
        }

        // Encode FPS and cell stats as colored pixel indicators
        // Green pixels = FPS (1 pixel per FPS unit), Yellow = cells redrawn ratio
        let fps_pixels = (info.fps as usize).min(w);
        for x in 0..fps_pixels {
            fb.set_pixel(x, 0, color::GREEN);
        }

        // Cells redrawn bar (row 1): proportional to redrawn/total
        if info.cells_total > 0 {
            let ratio_pixels = (info.cells_redrawn * w) / info.cells_total.max(1);
            for x in 0..ratio_pixels.min(w) {
                fb.set_pixel(x, 1, [255, 255, 0]); // yellow
            }
        }

        true
    });

    Ok(())
}
