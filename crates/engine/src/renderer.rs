use std::fmt::Write as FmtWrite;
use std::io::{self, Write};

use crate::color::Color;
use crate::framebuffer::FrameBuffer;

const HALF_BLOCK: char = '\u{2584}';
const BEGIN_SYNC: &str = "\x1b[?2026h";
const END_SYNC: &str = "\x1b[?2026l";

pub struct Renderer {
    buf: String,
}

impl Renderer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: String::with_capacity(capacity),
        }
    }

    /// Render the framebuffer to stdout using half-block characters.
    /// Each terminal row maps to two pixel rows (top = bg color, bottom = fg color).
    pub fn render(&mut self, fb: &FrameBuffer) -> io::Result<()> {
        let width = fb.width();
        let height = fb.height();
        if width == 0 || height == 0 {
            return Ok(());
        }
        let bg = fb.background();
        let term_rows = height / 2;

        self.buf.clear();

        // Begin synchronized update + move cursor home
        self.buf.push_str(BEGIN_SYNC);
        self.buf.push_str("\x1b[H");

        let mut cur_fg: Option<Color> = None;
        let mut cur_bg: Option<Color> = None;

        for row in 0..term_rows {
            if row > 0 {
                self.buf.push_str("\r\n");
            }
            for col in 0..width {
                let top = fb.get_pixel(col, row * 2).unwrap_or(bg);
                let bottom = fb.get_pixel(col, row * 2 + 1).unwrap_or(bg);

                // Set background (top pixel)
                if cur_bg != Some(top) {
                    let _ = write!(self.buf, "\x1b[48;2;{};{};{}m", top[0], top[1], top[2]);
                    cur_bg = Some(top);
                }

                // Set foreground (bottom pixel)
                if cur_fg != Some(bottom) {
                    let _ = write!(
                        self.buf,
                        "\x1b[38;2;{};{};{}m",
                        bottom[0], bottom[1], bottom[2]
                    );
                    cur_fg = Some(bottom);
                }

                self.buf.push(HALF_BLOCK);
            }
        }

        // Reset colors + end synchronized update
        self.buf.push_str("\x1b[0m");
        self.buf.push_str(END_SYNC);

        let mut stdout = io::stdout().lock();
        stdout.write_all(self.buf.as_bytes())?;
        stdout.flush()
    }
}
