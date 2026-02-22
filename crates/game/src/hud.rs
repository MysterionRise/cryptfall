use engine::color::Color;
use engine::FrameBuffer;

use crate::sprites::effects::{HEART_EMPTY, HEART_FULL};
use crate::sprites::font::render_digit;

pub struct DamageNumber {
    pub value: i32,
    pub x: f32,
    pub y: f32,
    velocity_y: f32,
    lifetime: f32,
    max_lifetime: f32,
    pub color: Color,
}

impl DamageNumber {
    pub fn new(value: i32, x: f32, y: f32, color: Color) -> Self {
        Self {
            value,
            x,
            y,
            velocity_y: -30.0, // float upward
            lifetime: 0.0,
            max_lifetime: 0.8,
            color,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime += dt;
        self.y += self.velocity_y * dt;
        self.velocity_y *= 0.95; // slow down
    }

    pub fn alive(&self) -> bool {
        self.lifetime < self.max_lifetime
    }

    pub fn render(&self, fb: &mut FrameBuffer, cam_x: i32, cam_y: i32) {
        // Fade based on remaining lifetime
        let fade = 1.0 - (self.lifetime / self.max_lifetime);
        let c = [
            (self.color[0] as f32 * fade) as u8,
            (self.color[1] as f32 * fade) as u8,
            (self.color[2] as f32 * fade) as u8,
        ];

        let sx = self.x as i32 - cam_x;
        let sy = self.y as i32 - cam_y;

        // Render each digit using the 3x5 pixel font
        let (digits, count) = get_digits(self.value);
        let mut offset_x = 0i32;
        for &digit in &digits[..count] {
            render_digit(fb, digit, sx + offset_x, sy, c);
            offset_x += 4; // 3 wide + 1 spacing
        }
    }
}

/// Returns digits of a non-negative integer as a fixed-size array.
/// Returns (array, count) where count is the number of valid digits.
fn get_digits(mut value: i32) -> ([u8; 5], usize) {
    if value <= 0 {
        let mut arr = [0u8; 5];
        arr[0] = 0;
        return (arr, 1);
    }
    let mut buf = [0u8; 5];
    let mut count = 0usize;
    while value > 0 && count < 5 {
        buf[count] = (value % 10) as u8;
        value /= 10;
        count += 1;
    }
    // Reverse the digits in-place
    buf[..count].reverse();
    (buf, count)
}

/// Render health hearts at screen position. Hearts are 5x5 with 1px spacing.
pub fn render_hearts(fb: &mut FrameBuffer, hp: i32, max_hp: i32, sx: i32, sy: i32) {
    let hp = hp.max(0);
    let max_hp = max_hp.max(0);
    for i in 0..max_hp {
        let x = sx + i * 6;
        let sprite = if i < hp { &HEART_FULL } else { &HEART_EMPTY };
        fb.blit_sprite(sprite, x, sy);
    }
}
