use engine::animation::AnimationData;
use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None; // transparent
const W: Option<Color> = Some([255, 255, 255]); // white
const S: Option<Color> = Some([240, 180, 140]); // skin
const H: Option<Color> = Some([139, 69, 19]); // hair/brown
const B: Option<Color> = Some([30, 100, 200]); // blue clothing

// =============================================================================
// IDLE ANIMATION — 2 frames, 0.5s per frame, looping
// Subtle breathing: frame 1 is shifted 1 pixel up from frame 0.
// =============================================================================

/// Idle frame 0: normal standing pose.
static IDLE_0: SpriteData = SpriteData::new(
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

/// Idle frame 1: shifted 1px up (breathing up).
static IDLE_1: SpriteData = SpriteData::new(
    8,
    12,
    &[
        N, H, H, H, H, H, H, N, // row 0: hair (was row 1)
        N, H, S, S, S, S, H, N, // row 1: face top (was row 2)
        N, N, S, W, S, W, N, N, // row 2: face eyes (was row 3)
        N, N, N, S, S, N, N, N, // row 3: neck (was row 4)
        N, B, B, B, B, B, B, N, // row 4: torso (was row 5)
        N, B, B, B, B, B, B, N, // row 5: torso (was row 6)
        N, B, B, B, B, B, B, N, // row 6: torso (was row 7)
        N, N, B, B, B, B, N, N, // row 7: waist (was row 8)
        N, N, B, N, N, B, N, N, // row 8: legs (was row 9)
        N, N, B, N, N, B, N, N, // row 9: legs (was row 10)
        N, N, H, N, N, H, N, N, // row 10: boots (was row 11)
        N, N, N, N, N, N, N, N, // row 11: empty (bob up)
    ],
);

pub static IDLE_ANIM: AnimationData = AnimationData {
    frames: &[&IDLE_0, &IDLE_1],
    frame_duration: 0.5,
    looping: true,
};

// =============================================================================
// WALK ANIMATION — 4 frames, 0.15s per frame, looping
// Frame 0: neutral (legs together)
// Frame 1: stride right, body 1px lower (bob down)
// Frame 2: neutral again
// Frame 3: stride left, body 1px lower (bob down)
// =============================================================================

/// Walk frame 0: neutral stance, legs together.
static WALK_0: SpriteData = SpriteData::new(
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
        N, N, N, B, B, N, N, N, // row 9: legs together
        N, N, N, B, B, N, N, N, // row 10: legs together
        N, N, N, H, H, N, N, N, // row 11: boots together
    ],
);

/// Walk frame 1: right stride, body 1px lower.
static WALK_1: SpriteData = SpriteData::new(
    8,
    12,
    &[
        N, N, N, N, N, N, N, N, // row 0: empty (bob down)
        N, N, H, H, H, H, N, N, // row 1: hair top
        N, H, H, H, H, H, H, N, // row 2: hair
        N, H, S, S, S, S, H, N, // row 3: face top
        N, N, S, W, S, W, N, N, // row 4: face (eyes)
        N, N, N, S, S, N, N, N, // row 5: neck
        N, B, B, B, B, B, B, N, // row 6: torso
        N, B, B, B, B, B, B, N, // row 7: torso
        N, B, B, B, B, B, B, N, // row 8: torso
        N, N, B, B, B, B, N, N, // row 9: waist
        N, B, N, N, N, N, B, N, // row 10: legs spread wide
        N, H, N, N, N, N, H, N, // row 11: boots spread
    ],
);

/// Walk frame 2: neutral stance (same as frame 0).
static WALK_2: SpriteData = SpriteData::new(
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
        N, N, N, B, B, N, N, N, // row 9: legs together
        N, N, N, B, B, N, N, N, // row 10: legs together
        N, N, N, H, H, N, N, N, // row 11: boots together
    ],
);

/// Walk frame 3: left stride, body 1px lower.
static WALK_3: SpriteData = SpriteData::new(
    8,
    12,
    &[
        N, N, N, N, N, N, N, N, // row 0: empty (bob down)
        N, N, H, H, H, H, N, N, // row 1: hair top
        N, H, H, H, H, H, H, N, // row 2: hair
        N, H, S, S, S, S, H, N, // row 3: face top
        N, N, S, W, S, W, N, N, // row 4: face (eyes)
        N, N, N, S, S, N, N, N, // row 5: neck
        N, B, B, B, B, B, B, N, // row 6: torso
        N, B, B, B, B, B, B, N, // row 7: torso
        N, B, B, B, B, B, B, N, // row 8: torso
        N, N, B, B, B, B, N, N, // row 9: waist
        N, N, B, N, B, N, N, N, // row 10: legs (opposite stride)
        N, N, H, N, H, N, N, N, // row 11: boots (opposite)
    ],
);

pub static WALK_ANIM: AnimationData = AnimationData {
    frames: &[&WALK_0, &WALK_1, &WALK_2, &WALK_3],
    frame_duration: 0.15,
    looping: true,
};

// =============================================================================
// DASH ANIMATION — 1 frame (crouched pose)
// =============================================================================

/// Dash frame: crouched, compact pose.
static DASH_0: SpriteData = SpriteData::new(
    8,
    12,
    &[
        N, N, N, N, N, N, N, N, // row 0: empty
        N, N, N, N, N, N, N, N, // row 1: empty
        N, N, H, H, H, H, N, N, // row 2: hair top (crouched down)
        N, H, H, H, H, H, H, N, // row 3: hair
        N, H, S, S, S, S, H, N, // row 4: face top
        N, N, S, W, S, W, N, N, // row 5: face (eyes)
        N, N, N, S, S, N, N, N, // row 6: neck
        N, B, B, B, B, B, B, N, // row 7: torso
        N, B, B, B, B, B, B, N, // row 8: torso
        N, B, B, B, B, B, B, N, // row 9: torso/waist
        N, B, N, N, N, N, B, N, // row 10: legs wide (braced)
        N, H, N, N, N, N, H, N, // row 11: boots wide
    ],
);

pub static DASH_ANIM: AnimationData = AnimationData {
    frames: &[&DASH_0],
    frame_duration: 1.0, // single frame, duration doesn't matter
    looping: false,
};
