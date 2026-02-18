use crate::color::Color;

/// Compile-time sprite data. Pixels are row-major; None = transparent.
pub struct SpriteData {
    pub width: usize,
    pub height: usize,
    pub pixels: &'static [Option<Color>],
}

impl SpriteData {
    pub const fn new(width: usize, height: usize, pixels: &'static [Option<Color>]) -> Self {
        SpriteData {
            width,
            height,
            pixels,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            None
        }
    }
}
