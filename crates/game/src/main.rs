use std::io::{self, Write};
use std::time::Duration;

use crossterm::{cursor, event, execute, style::Print, terminal};

fn main() -> io::Result<()> {
    let _terminal = engine::Terminal::new()?;

    engine::run(|fps| {
        let mut stdout = io::stdout();

        // Clear and draw FPS display
        let _ = execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            Print(format!("Cryptfall Engine - {} FPS", fps)),
            cursor::MoveTo(0, 1),
            Print("Press 'q' to quit"),
        );
        let _ = stdout.flush();

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
