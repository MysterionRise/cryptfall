pub mod animation;
pub mod camera;
pub mod collision;
pub mod color;
pub mod framebuffer;
pub mod gameloop;
pub mod input;
pub mod particle;
pub mod renderer;
pub mod sprite;
pub mod tilemap;
pub mod types;

use std::io;

use crossterm::{cursor, event, execute, terminal};

pub use animation::{AnimationData, AnimationPlayer};
pub use camera::Camera;
pub use collision::AABB;
pub use color::Color;
pub use framebuffer::FrameBuffer;
pub use particle::{BurstConfig, Particle, ParticleSystem};
pub use gameloop::{FrameInfo, Game};
pub use input::{GameKey, InputState};
pub use renderer::{RenderStats, Renderer};
pub use sprite::SpriteData;
pub use tilemap::{render_tilemap, TileMap, TileType, TILE_SIZE};
pub use types::{Transform, Vec2};

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
    /// Clamps to minimum 20×10 to prevent zero-size panics.
    pub fn handle_resize(&mut self, cols: u16, rows: u16) {
        let cols = cols.max(20) as usize;
        let rows = rows.max(10) as usize;
        self.fb.resize(cols, rows);
        self.renderer.resize(cols, rows);
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        cleanup_terminal();
    }
}

/// Run the game loop with the given Game implementation.
pub fn run(term: &mut Terminal, game: &mut dyn Game) {
    gameloop::run(term, game);
}
