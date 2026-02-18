use std::time::{Duration, Instant};

use crossterm::event;

use crate::framebuffer::FrameBuffer;
use crate::input::InputState;
use crate::renderer::RenderStats;
use crate::Terminal;

/// Fixed timestep: 30 ticks per second.
const TICK_RATE: f64 = 1.0 / 30.0;
/// Clamp frame time to prevent spiral of death after long freezes.
const MAX_FRAME_TIME: f64 = 0.25;

/// Frame information passed to the game each render call.
pub struct FrameInfo {
    pub fps: u32,
    pub cells_redrawn: usize,
    pub cells_total: usize,
    /// Time spent polling and processing input (microseconds).
    pub input_us: u64,
    /// Time spent in fixed update ticks (microseconds).
    pub update_us: u64,
    /// Time spent rendering to terminal (microseconds).
    pub render_us: u64,
}

/// Trait that games implement to receive fixed updates and interpolated renders.
pub trait Game {
    /// Called at a fixed 30Hz rate. `dt` is always TICK_RATE (~0.0333s).
    /// Return `false` to exit.
    fn update(&mut self, input: &InputState, dt: f64) -> bool;

    /// Called once per frame after all ticks. `alpha` is the interpolation
    /// factor (0.0–1.0) for smooth rendering between ticks.
    fn render(&mut self, fb: &mut FrameBuffer, info: &FrameInfo, alpha: f32);
}

/// Run the fixed-timestep game loop.
pub fn run(term: &mut Terminal, game: &mut dyn Game) {
    let mut accumulator: f64 = 0.0;
    let mut current_time = Instant::now();

    let mut fps: u32 = 0;
    let mut fps_frame_count: u32 = 0;
    let mut fps_timer = Instant::now();

    let mut last_stats = RenderStats {
        cells_redrawn: 0,
        cells_total: 0,
    };

    let mut events: Vec<event::Event> = Vec::with_capacity(16);
    let mut last_input_us: u64;
    let mut last_update_us: u64;
    let mut last_render_us: u64 = 0;

    loop {
        let new_time = Instant::now();
        let frame_time = new_time.duration_since(current_time).as_secs_f64();
        let frame_time = frame_time.min(MAX_FRAME_TIME);
        current_time = new_time;
        accumulator += frame_time;

        // --- Input phase ---
        let input_start = Instant::now();
        term.input.begin_frame();
        events.clear();
        while event::poll(Duration::ZERO).unwrap_or(false) {
            if let Ok(evt) = event::read() {
                if let event::Event::Resize(cols, rows) = evt {
                    term.handle_resize(cols, rows);
                }
                events.push(evt);
            }
        }
        term.input.process_events(&events);
        last_input_us = input_start.elapsed().as_micros() as u64;

        // --- Update phase (fixed-timestep) ---
        let update_start = Instant::now();
        while accumulator >= TICK_RATE {
            if !game.update(&term.input, TICK_RATE) {
                return;
            }
            accumulator -= TICK_RATE;
        }
        last_update_us = update_start.elapsed().as_micros() as u64;

        // Interpolation factor for smooth rendering
        let alpha = (accumulator / TICK_RATE) as f32;

        // --- Render phase ---
        let render_start = Instant::now();
        term.fb.clear();

        let info = FrameInfo {
            fps,
            cells_redrawn: last_stats.cells_redrawn,
            cells_total: last_stats.cells_total,
            input_us: last_input_us,
            update_us: last_update_us,
            render_us: last_render_us,
        };

        game.render(&mut term.fb, &info, alpha);

        if let Ok(stats) = term.renderer.render(&term.fb) {
            last_stats = stats;
        }
        last_render_us = render_start.elapsed().as_micros() as u64;

        // FPS calculation
        fps_frame_count += 1;
        let fps_elapsed = fps_timer.elapsed();
        if fps_elapsed >= Duration::from_secs(1) {
            fps = (fps_frame_count as f64 / fps_elapsed.as_secs_f64()).round() as u32;
            fps_frame_count = 0;
            fps_timer = Instant::now();
        }

        // Sleep for remaining frame budget
        let total_frame_time = Instant::now().duration_since(new_time).as_secs_f64();
        let sleep_time = TICK_RATE - total_frame_time - 0.001; // 1ms margin
        if sleep_time > 0.0 {
            std::thread::sleep(Duration::from_secs_f64(sleep_time));
        }
    }
}
