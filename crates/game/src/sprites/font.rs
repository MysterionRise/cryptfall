use engine::color::Color;
use engine::FrameBuffer;

/// 3x5 pixel font data for digits 0-9. Each digit is 15 bools (3 wide x 5 tall, row-major).
const DIGIT_FONTS: [[bool; 15]; 10] = [
    // 0
    [
        true, true, true, true, false, true, true, false, true, true, false, true, true, true,
        true,
    ],
    // 1
    [
        false, true, false, true, true, false, false, true, false, false, true, false, true, true,
        true,
    ],
    // 2
    [
        true, true, true, false, false, true, true, true, true, true, false, false, true, true,
        true,
    ],
    // 3
    [
        true, true, true, false, false, true, true, true, true, false, false, true, true, true,
        true,
    ],
    // 4
    [
        true, false, true, true, false, true, true, true, true, false, false, true, false, false,
        true,
    ],
    // 5
    [
        true, true, true, true, false, false, true, true, true, false, false, true, true, true,
        true,
    ],
    // 6
    [
        true, true, true, true, false, false, true, true, true, true, false, true, true, true,
        true,
    ],
    // 7
    [
        true, true, true, false, false, true, false, true, false, false, true, false, false, true,
        false,
    ],
    // 8
    [
        true, true, true, true, false, true, true, true, true, true, false, true, true, true,
        true,
    ],
    // 9
    [
        true, true, true, true, false, true, true, true, true, false, false, true, true, true,
        true,
    ],
];

/// Render a single digit at screen coordinates (sx, sy) with the given color.
/// Uses safe bounds checking via set_pixel.
pub fn render_digit(fb: &mut FrameBuffer, digit: u8, sx: i32, sy: i32, color: Color) {
    if digit > 9 {
        return;
    }
    let font = &DIGIT_FONTS[digit as usize];
    for row in 0..5 {
        for col in 0..3 {
            if font[row * 3 + col] {
                let px = sx + col as i32;
                let py = sy + row as i32;
                if px >= 0 && py >= 0 {
                    fb.set_pixel(px as usize, py as usize, color);
                }
            }
        }
    }
}
