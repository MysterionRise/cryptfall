use crate::color::{Color, DARK_GRAY};

pub struct FrameBuffer {
    width: usize,
    height: usize, // pixel rows = terminal rows * 2
    pixels: Vec<Option<Color>>,
    background: Color,
}

impl FrameBuffer {
    /// Create a new framebuffer. `term_cols` x `term_rows` are terminal dimensions.
    /// Pixel height is `term_rows * 2` (half-block rendering).
    pub fn new(term_cols: usize, term_rows: usize) -> Self {
        let width = term_cols;
        let height = term_rows * 2;
        Self {
            width,
            height,
            pixels: vec![None; width * height],
            background: DARK_GRAY,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn background(&self) -> Color {
        self.background
    }

    /// Clear all pixels to None (transparent / background).
    pub fn clear(&mut self) {
        self.pixels.fill(None);
    }

    /// Set a single pixel. Out-of-bounds writes are silently ignored.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = Some(color);
        }
    }

    /// Get a single pixel. Returns None for out-of-bounds or transparent pixels.
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            None
        }
    }

    /// Fill a rectangle with the given color. Clips to framebuffer bounds.
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        for row in y..y_end {
            let start = row * self.width + x.min(self.width);
            let end = row * self.width + x_end;
            for pixel in &mut self.pixels[start..end] {
                *pixel = Some(color);
            }
        }
    }

    /// Resize the framebuffer to new terminal dimensions. Clears all pixels.
    pub fn resize(&mut self, term_cols: usize, term_rows: usize) {
        self.width = term_cols;
        self.height = term_rows * 2;
        self.pixels.resize(self.width * self.height, None);
        self.pixels.fill(None);
    }
}
