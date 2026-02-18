use std::fmt::Write as FmtWrite;
use std::io::{self, Write};

use crate::color::Color;
use crate::framebuffer::FrameBuffer;

const HALF_BLOCK: char = '\u{2584}';
const BEGIN_SYNC: &str = "\x1b[?2026h";
const END_SYNC: &str = "\x1b[?2026l";

#[derive(Clone, Copy, PartialEq)]
struct Cell {
    top: Color,
    bottom: Color,
}

pub struct Renderer {
    front: Vec<Cell>,
    back: Vec<Cell>,
    width: usize,  // terminal columns
    height: usize, // terminal rows
    buf: String,
    force_redraw: bool,
}

/// Stats from the last render call.
pub struct RenderStats {
    pub cells_redrawn: usize,
    pub cells_total: usize,
}

impl Renderer {
    pub fn new(term_cols: usize, term_rows: usize) -> Self {
        let total = term_cols * term_rows;
        let default = Cell {
            top: [0, 0, 0],
            bottom: [0, 0, 0],
        };
        Self {
            front: vec![default; total],
            back: vec![default; total],
            width: term_cols,
            height: term_rows,
            buf: String::with_capacity(term_cols * term_rows * 30),
            force_redraw: true, // first frame is always a full draw
        }
    }

    /// Force a full redraw on the next render call.
    pub fn force_redraw(&mut self) {
        self.force_redraw = true;
    }

    /// Resize the renderer buffers. Forces a full redraw.
    pub fn resize(&mut self, term_cols: usize, term_rows: usize) {
        let total = term_cols * term_rows;
        let default = Cell {
            top: [0, 0, 0],
            bottom: [0, 0, 0],
        };
        self.width = term_cols;
        self.height = term_rows;
        self.front.resize(total, default);
        self.front.fill(default);
        self.back.resize(total, default);
        self.back.fill(default);
        self.force_redraw = true;
    }

    /// Render the framebuffer using diff rendering. Only changed cells are redrawn.
    pub fn render(&mut self, fb: &FrameBuffer) -> io::Result<RenderStats> {
        let width = self.width;
        let height = self.height;
        if width == 0 || height == 0 {
            return Ok(RenderStats {
                cells_redrawn: 0,
                cells_total: 0,
            });
        }
        let bg = fb.background();
        let total = width * height;

        // Convert framebuffer pixels to back buffer cells
        for row in 0..height {
            for col in 0..width {
                let top = fb.get_pixel(col, row * 2).unwrap_or(bg);
                let bottom = fb.get_pixel(col, row * 2 + 1).unwrap_or(bg);
                self.back[row * width + col] = Cell { top, bottom };
            }
        }

        self.buf.clear();
        self.buf.push_str(BEGIN_SYNC);

        let mut cells_redrawn = 0;
        let mut cur_fg: Option<Color> = None;
        let mut cur_bg: Option<Color> = None;
        // Track whether the cursor is already at the right position
        let mut cursor_col: usize = usize::MAX;
        let mut cursor_row: usize = usize::MAX;

        let force = self.force_redraw;

        for row in 0..height {
            for col in 0..width {
                let idx = row * width + col;
                let cell = self.back[idx];

                if !force && cell == self.front[idx] {
                    // Cell unchanged, cursor position is now unknown if we skipped
                    // (only matters if next cell needs drawing)
                    if cursor_row == row && cursor_col == col {
                        // We were tracking sequential, now we lost position
                        cursor_col = usize::MAX;
                        cursor_row = usize::MAX;
                    }
                    continue;
                }

                cells_redrawn += 1;

                // Position cursor if not already at the right spot
                if cursor_row != row || cursor_col != col {
                    // ANSI cursor positioning is 1-indexed
                    let _ = write!(self.buf, "\x1b[{};{}H", row + 1, col + 1);
                }

                // Set background (top pixel)
                if cur_bg != Some(cell.top) {
                    let _ = write!(
                        self.buf,
                        "\x1b[48;2;{};{};{}m",
                        cell.top[0], cell.top[1], cell.top[2]
                    );
                    cur_bg = Some(cell.top);
                }

                // Set foreground (bottom pixel)
                if cur_fg != Some(cell.bottom) {
                    let _ = write!(
                        self.buf,
                        "\x1b[38;2;{};{};{}m",
                        cell.bottom[0], cell.bottom[1], cell.bottom[2]
                    );
                    cur_fg = Some(cell.bottom);
                }

                self.buf.push(HALF_BLOCK);
                cursor_col = col + 1;
                cursor_row = row;
            }
        }

        self.buf.push_str("\x1b[0m");
        self.buf.push_str(END_SYNC);

        let mut stdout = io::stdout().lock();
        stdout.write_all(self.buf.as_bytes())?;
        stdout.flush()?;

        // Swap: copy back → front
        self.front.copy_from_slice(&self.back);
        self.force_redraw = false;

        Ok(RenderStats {
            cells_redrawn,
            cells_total: total,
        })
    }
}
