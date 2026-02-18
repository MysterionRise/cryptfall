use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None; // transparent
const W: Option<Color> = Some([255, 255, 255]); // white
const S: Option<Color> = Some([240, 180, 140]); // skin
const H: Option<Color> = Some([139, 69, 19]); // hair/brown
const B: Option<Color> = Some([30, 100, 200]); // blue clothing

/// 8×12 pixel test character (facing right).
pub static PLAYER_TEST: SpriteData = SpriteData::new(
    8,
    12,
    &[
        N, N, H, H, H, H, N, N, // row 0: hair top
        N, H, H, H, H, H, H, N, // row 1: hair
        N, H, S, S, S, S, H, N, // row 2: face top
        N, N, S, W, S, W, N, N, // row 3: face (eyes)
        N, N, N, S, S, N, N, N, // row 4: neck
        N, B, B, B, B, B, B, N, // row 5: torso
        N, B, B, B, B, B, B, N, // row 6: torso
        N, B, B, B, B, B, B, N, // row 7: torso
        N, N, B, B, B, B, N, N, // row 8: waist
        N, N, B, N, N, B, N, N, // row 9: legs
        N, N, B, N, N, B, N, N, // row 10: legs
        N, N, H, N, N, H, N, N, // row 11: boots
    ],
);
