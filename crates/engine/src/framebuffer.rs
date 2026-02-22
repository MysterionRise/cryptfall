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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_correct_dimensions() {
        // Arrange & Act
        let fb = FrameBuffer::new(80, 24);

        // Assert
        assert_eq!(fb.width(), 80, "Width should match term_cols");
        assert_eq!(
            fb.height(),
            48,
            "Height should be term_rows * 2 (half-block rendering)"
        );
    }

    #[test]
    fn test_new_all_pixels_none() {
        // Arrange & Act
        let fb = FrameBuffer::new(10, 5);

        // Assert: all pixels should be None
        for x in 0..10 {
            for y in 0..10 {
                assert_eq!(
                    fb.get_pixel(x, y),
                    None,
                    "Pixel ({}, {}) should be None after creation",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_set_pixel_and_get_pixel_roundtrip() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5);
        let color: Color = [128, 64, 32];

        // Act
        fb.set_pixel(3, 4, color);

        // Assert
        assert_eq!(
            fb.get_pixel(3, 4),
            Some(color),
            "get_pixel should return the color set by set_pixel"
        );
    }

    #[test]
    fn test_set_pixel_multiple_colors() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5);
        let red: Color = [255, 0, 0];
        let green: Color = [0, 255, 0];
        let blue: Color = [0, 0, 255];

        // Act
        fb.set_pixel(0, 0, red);
        fb.set_pixel(1, 1, green);
        fb.set_pixel(2, 2, blue);

        // Assert
        assert_eq!(fb.get_pixel(0, 0), Some(red), "Should get red at (0,0)");
        assert_eq!(fb.get_pixel(1, 1), Some(green), "Should get green at (1,1)");
        assert_eq!(fb.get_pixel(2, 2), Some(blue), "Should get blue at (2,2)");
    }

    #[test]
    fn test_set_pixel_overwrites_previous() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5);

        // Act
        fb.set_pixel(5, 5, [100, 100, 100]);
        fb.set_pixel(5, 5, [200, 200, 200]);

        // Assert
        assert_eq!(
            fb.get_pixel(5, 5),
            Some([200, 200, 200]),
            "Second set_pixel should overwrite the first"
        );
    }

    #[test]
    fn test_set_pixel_out_of_bounds_is_silently_ignored() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5); // 10 wide, 10 tall (5*2)

        // Act: these should not panic
        fb.set_pixel(100, 0, [255, 0, 0]);
        fb.set_pixel(0, 100, [255, 0, 0]);
        fb.set_pixel(100, 100, [255, 0, 0]);
        fb.set_pixel(usize::MAX, usize::MAX, [255, 0, 0]);

        // Assert: no panic occurred (implicit), and in-bounds pixels unaffected
        assert_eq!(fb.get_pixel(0, 0), None, "In-bounds pixel should be unaffected");
    }

    #[test]
    fn test_get_pixel_out_of_bounds_returns_none() {
        // Arrange
        let fb = FrameBuffer::new(10, 5); // 10 wide, 10 tall

        // Act & Assert
        assert_eq!(
            fb.get_pixel(100, 0),
            None,
            "Out-of-bounds x should return None"
        );
        assert_eq!(
            fb.get_pixel(0, 100),
            None,
            "Out-of-bounds y should return None"
        );
        assert_eq!(
            fb.get_pixel(100, 100),
            None,
            "Both out-of-bounds should return None"
        );
    }

    #[test]
    fn test_set_pixel_safe_with_negative_coordinates_does_not_panic() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5);

        // Act: negative coordinates should be silently ignored
        fb.set_pixel_safe(-1, 0, [255, 0, 0]);
        fb.set_pixel_safe(0, -1, [255, 0, 0]);
        fb.set_pixel_safe(-100, -100, [255, 0, 0]);
        fb.set_pixel_safe(i32::MIN, i32::MIN, [255, 0, 0]);

        // Assert: no panic (implicit), in-bounds pixels unaffected
        assert_eq!(fb.get_pixel(0, 0), None);
    }

    #[test]
    fn test_set_pixel_safe_with_valid_coordinates_works() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5);

        // Act
        fb.set_pixel_safe(5, 5, [42, 42, 42]);

        // Assert
        assert_eq!(
            fb.get_pixel(5, 5),
            Some([42, 42, 42]),
            "set_pixel_safe with valid coordinates should work like set_pixel"
        );
    }

    #[test]
    fn test_set_pixel_safe_out_of_bounds_positive_ignored() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5);

        // Act: large positive coordinates beyond bounds
        fb.set_pixel_safe(1000, 0, [255, 0, 0]);
        fb.set_pixel_safe(0, 1000, [255, 0, 0]);

        // Assert: no panic (implicit)
        assert_eq!(fb.get_pixel(0, 0), None);
    }

    #[test]
    fn test_clear_sets_all_pixels_to_none() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5);
        // Set some pixels
        fb.set_pixel(0, 0, [255, 0, 0]);
        fb.set_pixel(5, 5, [0, 255, 0]);
        fb.set_pixel(9, 9, [0, 0, 255]);

        // Act
        fb.clear();

        // Assert
        assert_eq!(fb.get_pixel(0, 0), None, "Pixel (0,0) should be None after clear");
        assert_eq!(fb.get_pixel(5, 5), None, "Pixel (5,5) should be None after clear");
        assert_eq!(fb.get_pixel(9, 9), None, "Pixel (9,9) should be None after clear");
    }

    #[test]
    fn test_clear_full_framebuffer() {
        // Arrange
        let mut fb = FrameBuffer::new(5, 3); // 5 wide, 6 tall
        // Fill every pixel
        for x in 0..5 {
            for y in 0..6 {
                fb.set_pixel(x, y, [255, 255, 255]);
            }
        }

        // Act
        fb.clear();

        // Assert
        for x in 0..5 {
            for y in 0..6 {
                assert_eq!(
                    fb.get_pixel(x, y),
                    None,
                    "Pixel ({}, {}) should be None after clear",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_resize_changes_dimensions_and_clears() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5);
        fb.set_pixel(0, 0, [255, 0, 0]);

        // Act
        fb.resize(20, 10);

        // Assert
        assert_eq!(fb.width(), 20, "Width should update to new term_cols");
        assert_eq!(
            fb.height(),
            20,
            "Height should be new term_rows * 2"
        );
        assert_eq!(
            fb.get_pixel(0, 0),
            None,
            "All pixels should be cleared after resize"
        );
    }

    #[test]
    fn test_resize_to_smaller() {
        // Arrange
        let mut fb = FrameBuffer::new(80, 24);

        // Act
        fb.resize(40, 12);

        // Assert
        assert_eq!(fb.width(), 40);
        assert_eq!(fb.height(), 24); // 12 * 2
    }

    #[test]
    fn test_fill_rect_basic() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5);
        let color: Color = [100, 200, 50];

        // Act: fill a 3x3 rect starting at (2,2)
        fb.fill_rect(2, 2, 3, 3, color);

        // Assert: pixels inside rect should be set
        for x in 2..5 {
            for y in 2..5 {
                assert_eq!(
                    fb.get_pixel(x, y),
                    Some(color),
                    "Pixel ({}, {}) should be filled",
                    x,
                    y
                );
            }
        }
        // Pixels outside should be None
        assert_eq!(fb.get_pixel(1, 2), None, "Pixel outside rect should be None");
        assert_eq!(fb.get_pixel(5, 2), None, "Pixel outside rect should be None");
    }

    #[test]
    fn test_boundary_pixels_at_max_indices() {
        // Arrange
        let mut fb = FrameBuffer::new(10, 5); // 10 wide, 10 tall

        // Act: set pixels at the boundary
        fb.set_pixel(9, 9, [255, 255, 255]); // last valid pixel
        fb.set_pixel(0, 0, [128, 128, 128]); // first valid pixel

        // Assert
        assert_eq!(fb.get_pixel(9, 9), Some([255, 255, 255]));
        assert_eq!(fb.get_pixel(0, 0), Some([128, 128, 128]));

        // Just beyond boundary
        assert_eq!(fb.get_pixel(10, 0), None);
        assert_eq!(fb.get_pixel(0, 10), None);
    }

    #[test]
    fn test_framebuffer_1x1() {
        // Arrange: minimal framebuffer (1 col, 1 row = 1x2 pixels)
        let mut fb = FrameBuffer::new(1, 1);

        // Assert dimensions
        assert_eq!(fb.width(), 1);
        assert_eq!(fb.height(), 2);

        // Act
        fb.set_pixel(0, 0, [10, 20, 30]);
        fb.set_pixel(0, 1, [40, 50, 60]);

        // Assert
        assert_eq!(fb.get_pixel(0, 0), Some([10, 20, 30]));
        assert_eq!(fb.get_pixel(0, 1), Some([40, 50, 60]));
        assert_eq!(fb.get_pixel(0, 2), None);
        assert_eq!(fb.get_pixel(1, 0), None);
    }
}
