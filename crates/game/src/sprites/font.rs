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

// 3x5 pixel letter fonts for text rendering
#[rustfmt::skip]
const LETTER_FONTS: [(char, [bool; 15]); 13] = [
    ('A', [false,true,false, true,false,true, true,true,true, true,false,true, true,false,true]),
    ('C', [false,true,true, true,false,false, true,false,false, true,false,false, false,true,true]),
    ('D', [true,true,false, true,false,true, true,false,true, true,false,true, true,true,false]),
    ('E', [true,true,true, true,false,false, true,true,false, true,false,false, true,true,true]),
    ('I', [true,true,true, false,true,false, false,true,false, false,true,false, true,true,true]),
    ('K', [true,false,true, true,false,true, true,true,false, true,false,true, true,false,true]),
    ('O', [false,true,false, true,false,true, true,false,true, true,false,true, false,true,false]),
    ('P', [true,true,false, true,false,true, true,true,false, true,false,false, true,false,false]),
    ('R', [true,true,false, true,false,true, true,true,false, true,false,true, true,false,true]),
    ('S', [false,true,true, true,false,false, false,true,false, false,false,true, true,true,false]),
    ('T', [true,true,true, false,true,false, false,true,false, false,true,false, false,true,false]),
    ('U', [true,false,true, true,false,true, true,false,true, true,false,true, false,true,false]),
    ('Y', [true,false,true, true,false,true, false,true,false, false,true,false, false,true,false]),
];

fn char_font(c: char) -> Option<&'static [bool; 15]> {
    if c.is_ascii_digit() {
        return Some(&DIGIT_FONTS[(c as u8 - b'0') as usize]);
    }
    for &(ch, ref font) in &LETTER_FONTS {
        if ch == c {
            return Some(font);
        }
    }
    None
}

/// Render a text string at screen coordinates using the 3x5 pixel font.
pub fn render_text(fb: &mut FrameBuffer, text: &str, sx: i32, sy: i32, color: Color) {
    let mut x = sx;
    for c in text.chars() {
        if c == ' ' {
            x += 4;
            continue;
        }
        if let Some(font) = char_font(c) {
            for row in 0..5 {
                for col in 0..3 {
                    if font[row * 3 + col] {
                        fb.set_pixel_safe(x + col as i32, sy + row as i32, color);
                    }
                }
            }
            x += 4;
        }
    }
}

/// Returns the width in pixels of a rendered text string.
pub fn text_width(text: &str) -> i32 {
    let chars = text.len() as i32;
    if chars == 0 { 0 } else { chars * 4 - 1 }
}
