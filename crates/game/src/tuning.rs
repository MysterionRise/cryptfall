//! Centralized gameplay tuning constants.
//!
//! All player-visible gameplay parameters live here for easy balancing.
//! Particle burst configs remain near their usage (combat.rs, main.rs).

use engine::Color;

// --- Visual feedback timing ---

/// Duration of the attack flash overlay in frames (at 30 FPS)
pub const FLASH_FRAMES: u32 = 5;

/// Seconds of player idle before demo mode engages
pub const DEMO_IDLE_THRESHOLD: f32 = 5.0;

/// Duration of the death fade-to-black in seconds
pub const DEATH_FADE_DURATION: f32 = 1.5;

// --- Tint colors ---

/// Dash i-frames tint: cool blue
pub const DASH_TINT: Color = [100, 160, 255];

/// Attack hit flash tint: warm red
pub const ATTACK_TINT: Color = [255, 80, 80];

/// I-frame flash tint: bright white
pub const IFRAME_TINT: Color = [255, 255, 255];
