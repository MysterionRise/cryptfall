pub mod color;
pub mod framebuffer;
pub mod input;
pub mod renderer;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{cursor, event, execute, terminal};

pub use color::Color;
pub use crossterm::event::Event;
pub use framebuffer::FrameBuffer;
pub use input::{GameKey, InputState};
pub use renderer::{RenderStats, Renderer};

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
pub struct Terminal {
    pub fb: FrameBuffer,
    pub renderer: Renderer,
    pub input: InputState,
}

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
        let (cols, rows) = terminal::size()?;
        let fb = FrameBuffer::new(cols as usize, rows as usize);
        let renderer = Renderer::new(cols as usize, rows as usize);
        let input = InputState::new();
        Ok(Terminal {
            fb,
            renderer,
            input,
        })
    }

    /// Returns (columns, rows).
    pub fn size() -> io::Result<(u16, u16)> {
        terminal::size()
    }

    /// Handle terminal resize: update framebuffer and renderer dimensions.
    pub fn handle_resize(&mut self, cols: u16, rows: u16) {
        self.fb.resize(cols as usize, rows as usize);
        self.renderer.resize(cols as usize, rows as usize);
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        cleanup_terminal();
    }
}

/// Frame information passed to the tick callback.
pub struct FrameInfo {
    pub fps: u32,
    pub cells_redrawn: usize,
    pub cells_total: usize,
}

/// Runs a game loop at ~30 FPS. The callback receives the framebuffer, input state,
/// and frame info. Return `false` to exit.
pub fn run<F>(term: &mut Terminal, mut tick: F)
where
    F: FnMut(&mut FrameBuffer, &InputState, &FrameInfo) -> bool,
{
    let mut frame_count: u32 = 0;
    let mut fps: u32 = 0;
    let mut fps_timer = Instant::now();
    let mut last_stats = RenderStats {
        cells_redrawn: 0,
        cells_total: 0,
    };
    let mut events: Vec<Event> = Vec::with_capacity(16);

    loop {
        let frame_start = Instant::now();

        // Begin input frame (check for timed-out held keys)
        term.input.begin_frame();

        // Drain all pending events
        events.clear();
        while event::poll(Duration::ZERO).unwrap_or(false) {
            if let Ok(evt) = event::read() {
                if let event::Event::Resize(cols, rows) = evt {
                    term.handle_resize(cols, rows);
                }
                events.push(evt);
            }
        }

        // Feed events into input system
        term.input.process_events(&events);

        term.fb.clear();

        let info = FrameInfo {
            fps,
            cells_redrawn: last_stats.cells_redrawn,
            cells_total: last_stats.cells_total,
        };

        if !tick(&mut term.fb, &term.input, &info) {
            break;
        }

        if let Ok(stats) = term.renderer.render(&term.fb) {
            last_stats = stats;
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
