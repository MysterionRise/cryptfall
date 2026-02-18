use std::io;
use std::time::{Duration, Instant};

use crossterm::{cursor, event, execute, terminal};

const TARGET_FPS: u32 = 30;
const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

/// Performs terminal cleanup. Safe to call multiple times.
fn cleanup_terminal() {
    let mut stdout = io::stdout();
    let _ = execute!(
        stdout,
        event::DisableMouseCapture,
        cursor::Show,
        terminal::LeaveAlternateScreen,
    );
    let _ = terminal::disable_raw_mode();
}

/// Installs a panic hook that restores the terminal before printing the panic info.
fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        cleanup_terminal();
        default_hook(info);
    }));
}

/// Manages raw-mode terminal lifetime. Restores terminal state on drop.
pub struct Terminal;

impl Terminal {
    /// Enter raw mode, switch to alternate screen, hide cursor, enable mouse capture.
    pub fn new() -> io::Result<Self> {
        install_panic_hook();
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            event::EnableMouseCapture,
        )?;
        Ok(Terminal)
    }

    /// Returns (columns, rows).
    pub fn size() -> io::Result<(u16, u16)> {
        terminal::size()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        cleanup_terminal();
    }
}

/// Runs a game loop at ~30 FPS. The callback receives the current FPS and returns
/// `false` to exit.
pub fn run<F>(mut tick: F)
where
    F: FnMut(u32) -> bool,
{
    let mut frame_count: u32 = 0;
    let mut fps: u32 = 0;
    let mut fps_timer = Instant::now();

    loop {
        let frame_start = Instant::now();

        if !tick(fps) {
            break;
        }

        frame_count += 1;
        let elapsed = fps_timer.elapsed();
        if elapsed >= Duration::from_secs(1) {
            fps = frame_count;
            frame_count = 0;
            fps_timer = Instant::now();
        }

        let frame_time = frame_start.elapsed();
        if frame_time < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - frame_time);
        }
    }
}
