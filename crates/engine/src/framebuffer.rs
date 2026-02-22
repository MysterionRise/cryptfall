use crate::color::{Color, DARK_GRAY};
use crate::sprite::SpriteData;

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

    /// Set a single pixel using signed coordinates. Silently ignores out-of-bounds.
    /// Useful for particles that can drift off-screen.
    pub fn set_pixel_safe(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 {
            let ux = x as usize;
            let uy = y as usize;
            if ux < self.width && uy < self.height {
                self.pixels[uy * self.width + ux] = Some(color);
            }
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

    /// Blit a sprite at pixel position (px, py).
    /// Handles clipping for partially off-screen sprites.
    /// Transparent pixels (None) are skipped.
    pub fn blit_sprite(&mut self, sprite: &SpriteData, px: i32, py: i32) {
        let (src_x0, dst_x0, w) = clip_axis(px, sprite.width, self.width);
        let (src_y0, dst_y0, h) = clip_axis(py, sprite.height, self.height);

        for row in 0..h {
            let sy = src_y0 + row;
            let dy = dst_y0 + row;
            for col in 0..w {
                let sx = src_x0 + col;
                if let Some(c) = sprite.pixels[sy * sprite.width + sx] {
                    self.pixels[dy * self.width + (dst_x0 + col)] = Some(c);
                }
            }
        }
    }

    /// Blit with horizontal flip (for left-facing sprites).
    pub fn blit_sprite_flipped(&mut self, sprite: &SpriteData, px: i32, py: i32) {
        let (src_x0, dst_x0, w) = clip_axis(px, sprite.width, self.width);
        let (src_y0, dst_y0, h) = clip_axis(py, sprite.height, self.height);

        for row in 0..h {
            let sy = src_y0 + row;
            let dy = dst_y0 + row;
            for col in 0..w {
                let sx = src_x0 + col;
                // Mirror: read from the opposite side of the sprite
                let flipped_sx = sprite.width - 1 - sx;
                if let Some(c) = sprite.pixels[sy * sprite.width + flipped_sx] {
                    self.pixels[dy * self.width + (dst_x0 + col)] = Some(c);
                }
            }
        }
    }

    /// Blit with a color tint (multiply each pixel channel by tint/255).
    /// Useful for damage flash (red tint) or ghost trail (reduced brightness).
    pub fn blit_sprite_tinted(&mut self, sprite: &SpriteData, px: i32, py: i32, tint: Color) {
        let (src_x0, dst_x0, w) = clip_axis(px, sprite.width, self.width);
        let (src_y0, dst_y0, h) = clip_axis(py, sprite.height, self.height);

        for row in 0..h {
            let sy = src_y0 + row;
            let dy = dst_y0 + row;
            for col in 0..w {
                let sx = src_x0 + col;
                if let Some(c) = sprite.pixels[sy * sprite.width + sx] {
                    let tinted = [
                        (c[0] as u16 * tint[0] as u16 / 255) as u8,
                        (c[1] as u16 * tint[1] as u16 / 255) as u8,
                        (c[2] as u16 * tint[2] as u16 / 255) as u8,
                    ];
                    self.pixels[dy * self.width + (dst_x0 + col)] = Some(tinted);
                }
            }
        }
    }

    /// Blit all non-transparent pixels as a solid color (for white hit flash).
    pub fn blit_sprite_solid(&mut self, sprite: &SpriteData, px: i32, py: i32, color: Color) {
        let (src_x0, dst_x0, w) = clip_axis(px, sprite.width, self.width);
        let (src_y0, dst_y0, h) = clip_axis(py, sprite.height, self.height);

        for row in 0..h {
            let sy = src_y0 + row;
            let dy = dst_y0 + row;
            for col in 0..w {
                let sx = src_x0 + col;
                if sprite.pixels[sy * sprite.width + sx].is_some() {
                    self.pixels[dy * self.width + (dst_x0 + col)] = Some(color);
                }
            }
        }
    }

    /// Blit with horizontal flip, all non-transparent pixels as a solid color.
    pub fn blit_sprite_flipped_solid(
        &mut self,
        sprite: &SpriteData,
        px: i32,
        py: i32,
        color: Color,
    ) {
        let (src_x0, dst_x0, w) = clip_axis(px, sprite.width, self.width);
        let (src_y0, dst_y0, h) = clip_axis(py, sprite.height, self.height);

        for row in 0..h {
            let sy = src_y0 + row;
            let dy = dst_y0 + row;
            for col in 0..w {
                let sx = src_x0 + col;
                let flipped_sx = sprite.width - 1 - sx;
                if sprite.pixels[sy * sprite.width + flipped_sx].is_some() {
                    self.pixels[dy * self.width + (dst_x0 + col)] = Some(color);
                }
            }
        }
    }

    /// Blit with horizontal flip and a color tint combined.
    pub fn blit_sprite_flipped_tinted(
        &mut self,
        sprite: &SpriteData,
        px: i32,
        py: i32,
        tint: Color,
    ) {
        let (src_x0, dst_x0, w) = clip_axis(px, sprite.width, self.width);
        let (src_y0, dst_y0, h) = clip_axis(py, sprite.height, self.height);

        for row in 0..h {
            let sy = src_y0 + row;
            let dy = dst_y0 + row;
            for col in 0..w {
                let sx = src_x0 + col;
                let flipped_sx = sprite.width - 1 - sx;
                if let Some(c) = sprite.pixels[sy * sprite.width + flipped_sx] {
                    let tinted = [
                        (c[0] as u16 * tint[0] as u16 / 255) as u8,
                        (c[1] as u16 * tint[1] as u16 / 255) as u8,
                        (c[2] as u16 * tint[2] as u16 / 255) as u8,
                    ];
                    self.pixels[dy * self.width + (dst_x0 + col)] = Some(tinted);
                }
            }
        }
    }

    /// Blend all non-transparent pixels toward a target color.
    /// `opacity` ranges from 0.0 (no change) to 1.0 (fully replaced by color).
    pub fn overlay(&mut self, color: Color, opacity: f32) {
        let opacity = opacity.clamp(0.0, 1.0);
        for c in self.pixels.iter_mut().flatten() {
            c[0] = (c[0] as f32 + (color[0] as f32 - c[0] as f32) * opacity) as u8;
            c[1] = (c[1] as f32 + (color[1] as f32 - c[1] as f32) * opacity) as u8;
            c[2] = (c[2] as f32 + (color[2] as f32 - c[2] as f32) * opacity) as u8;
        }
    }
}

/// Calculate clipped source and destination ranges for one axis.
/// Returns (src_start, dst_start, count) — the visible region.
fn clip_axis(pos: i32, sprite_size: usize, fb_size: usize) -> (usize, usize, usize) {
    let src_start = if pos < 0 { (-pos) as usize } else { 0 };
    let dst_start = pos.max(0) as usize;
    let end = (pos + sprite_size as i32).min(fb_size as i32);
    if end <= 0 {
        return (0, 0, 0);
    }
    let count = (end as usize).saturating_sub(dst_start);
    (src_start, dst_start, count)
}
