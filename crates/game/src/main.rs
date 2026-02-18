use std::time::Duration;

use crossterm::event;
use engine::FrameBuffer;

fn main() -> std::io::Result<()> {
    let mut terminal = engine::Terminal::new()?;

    engine::run(&mut terminal, |fb: &mut FrameBuffer, _fps| {
        let w = fb.width();
        let h = fb.height();

        // Draw RGB gradient: red varies across X, green varies across Y
        for y in 0..h {
            for x in 0..w {
                let r = if w > 1 { (x * 255 / (w - 1)) as u8 } else { 0 };
                let g = if h > 1 { (y * 255 / (h - 1)) as u8 } else { 0 };
                let b = 80;
                fb.set_pixel(x, y, [r, g, b]);
            }
        }

        // Non-blocking input poll
        if event::poll(Duration::ZERO).unwrap_or(false) {
            if let Ok(event::Event::Key(key)) = event::read() {
                if key.code == event::KeyCode::Char('q') {
                    return false;
                }
            }
        }

        true
    });

    Ok(())
}
