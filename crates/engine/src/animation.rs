use crate::sprite::SpriteData;

/// A sequence of sprite frames with timing.
pub struct AnimationData {
    pub frames: &'static [&'static SpriteData],
    pub frame_duration: f64,
    pub looping: bool,
}

/// Runtime animation state.
pub struct AnimationPlayer {
    current_animation: &'static AnimationData,
    current_frame: usize,
    elapsed: f64,
    finished: bool,
    flipped: bool,
}

impl AnimationPlayer {
    pub fn new(animation: &'static AnimationData) -> Self {
        Self {
            current_animation: animation,
            current_frame: 0,
            elapsed: 0.0,
            finished: false,
            flipped: false,
        }
    }

    /// Switch to a new animation. Resets frame to 0.
    /// If already playing this animation, does nothing (prevents restart stutter).
    pub fn play(&mut self, animation: &'static AnimationData) {
        if std::ptr::eq(self.current_animation, animation) {
            return;
        }
        self.current_animation = animation;
        self.current_frame = 0;
        self.elapsed = 0.0;
        self.finished = false;
    }

    /// Advance animation by dt seconds.
    pub fn update(&mut self, dt: f64) {
        if self.finished {
            return;
        }
        self.elapsed += dt;
        while self.elapsed >= self.current_animation.frame_duration {
            self.elapsed -= self.current_animation.frame_duration;
            self.current_frame += 1;
            if self.current_frame >= self.current_animation.frames.len() {
                if self.current_animation.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.current_animation.frames.len() - 1;
                    self.finished = true;
                    return;
                }
            }
        }
    }

    /// Get the current frame's sprite data.
    /// Returns the last frame if index is somehow out of bounds (safety guard).
    pub fn current_sprite(&self) -> &'static SpriteData {
        let len = self.current_animation.frames.len();
        if len == 0 {
            panic!("Animation has no frames");
        }
        self.current_animation.frames[self.current_frame.min(len - 1)]
    }

    /// Is a one-shot animation finished?
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.flipped = flipped;
    }

    pub fn is_flipped(&self) -> bool {
        self.flipped
    }

    /// Get the current frame index within the animation.
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    // --- Test fixtures: minimal static sprites and animations ---

    static PIXEL_A: [Option<Color>; 1] = [Some([255, 0, 0])];
    static PIXEL_B: [Option<Color>; 1] = [Some([0, 255, 0])];
    static PIXEL_C: [Option<Color>; 1] = [Some([0, 0, 255])];

    static SPRITE_A: SpriteData = SpriteData {
        width: 1,
        height: 1,
        pixels: &PIXEL_A,
    };
    static SPRITE_B: SpriteData = SpriteData {
        width: 1,
        height: 1,
        pixels: &PIXEL_B,
    };
    static SPRITE_C: SpriteData = SpriteData {
        width: 1,
        height: 1,
        pixels: &PIXEL_C,
    };

    static FRAMES_3: [&SpriteData; 3] = [&SPRITE_A, &SPRITE_B, &SPRITE_C];

    static ANIM_LOOPING: AnimationData = AnimationData {
        frames: &FRAMES_3,
        frame_duration: 0.1,
        looping: true,
    };

    static ANIM_ONESHOT: AnimationData = AnimationData {
        frames: &FRAMES_3,
        frame_duration: 0.1,
        looping: false,
    };

    static FRAMES_ALT: [&SpriteData; 2] = [&SPRITE_B, &SPRITE_A];

    static ANIM_ALT: AnimationData = AnimationData {
        frames: &FRAMES_ALT,
        frame_duration: 0.2,
        looping: true,
    };

    #[test]
    fn test_animation_starts_at_frame_zero() {
        // Arrange & Act
        let player = AnimationPlayer::new(&ANIM_LOOPING);

        // Assert
        assert_eq!(player.current_frame(), 0, "New player should start at frame 0");
        assert!(!player.is_finished(), "New player should not be finished");
    }

    #[test]
    fn test_animation_frame_advances_on_tick() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);

        // Act: advance by one full frame duration
        player.update(0.1);

        // Assert
        assert_eq!(
            player.current_frame(),
            1,
            "Frame should advance to 1 after one frame_duration"
        );
    }

    #[test]
    fn test_animation_frame_does_not_advance_before_duration() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);

        // Act: advance by less than one frame duration
        player.update(0.05);

        // Assert
        assert_eq!(
            player.current_frame(),
            0,
            "Frame should stay at 0 when dt < frame_duration"
        );
    }

    #[test]
    fn test_animation_multiple_frames_in_one_update() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);

        // Act: advance by enough for 2 frames
        player.update(0.2);

        // Assert
        assert_eq!(
            player.current_frame(),
            2,
            "Should advance multiple frames in a single large dt"
        );
    }

    #[test]
    fn test_looping_animation_wraps_to_frame_zero() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);

        // Act: advance past all 3 frames. Use 0.31 to avoid floating-point
        // precision issues (0.3 in f64 is slightly less than 3 * 0.1).
        player.update(0.31);

        // Assert
        assert_eq!(
            player.current_frame(),
            0,
            "Looping animation should wrap to frame 0 after cycling through all frames"
        );
        assert!(
            !player.is_finished(),
            "Looping animation should never be finished"
        );
    }

    #[test]
    fn test_looping_animation_wraps_multiple_cycles() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);

        // Act: advance by enough for 7+ frames. Using 0.71 to avoid floating-point
        // precision issues where 0.7 - 7*0.1 might not cross the threshold cleanly.
        // 7 full frame advances: 7 % 3 = frame 1
        player.update(0.71);

        // Assert
        assert_eq!(
            player.current_frame(),
            1,
            "After 7 frames in a 3-frame loop, should be at frame 1"
        );
    }

    #[test]
    fn test_non_looping_animation_sets_finished_flag() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_ONESHOT);

        // Act: advance well past all 3 frames (use 0.31 to avoid f64 precision issues)
        player.update(0.31);

        // Assert
        assert!(
            player.is_finished(),
            "One-shot animation should be finished after playing all frames"
        );
    }

    #[test]
    fn test_non_looping_animation_stays_on_last_frame() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_ONESHOT);

        // Act
        player.update(0.3);
        let frame_after_finish = player.current_frame();
        player.update(1.0); // extra time should have no effect
        let frame_after_extra = player.current_frame();

        // Assert
        assert_eq!(
            frame_after_finish, 2,
            "One-shot should stay on last frame (index 2)"
        );
        assert_eq!(
            frame_after_extra, 2,
            "Additional updates should not change frame after finish"
        );
    }

    #[test]
    fn test_frame_index_never_exceeds_frame_count() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_ONESHOT);

        // Act: advance by a huge amount
        player.update(100.0);

        // Assert
        assert!(
            player.current_frame() < ANIM_ONESHOT.frames.len(),
            "Frame index ({}) should never reach or exceed frame count ({})",
            player.current_frame(),
            ANIM_ONESHOT.frames.len()
        );
    }

    #[test]
    fn test_current_sprite_returns_correct_frame() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);

        // Act & Assert: frame 0 should be SPRITE_A (red)
        let sprite0 = player.current_sprite();
        assert_eq!(
            sprite0.pixels[0],
            Some([255, 0, 0]),
            "Frame 0 should return SPRITE_A (red)"
        );

        // Advance to frame 1 (SPRITE_B, green)
        player.update(0.1);
        let sprite1 = player.current_sprite();
        assert_eq!(
            sprite1.pixels[0],
            Some([0, 255, 0]),
            "Frame 1 should return SPRITE_B (green)"
        );
    }

    #[test]
    fn test_play_same_animation_does_not_reset() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);
        player.update(0.15); // advance to frame 1

        // Act: play the same animation again
        player.play(&ANIM_LOOPING);

        // Assert
        assert_eq!(
            player.current_frame(),
            1,
            "Playing the same animation should not reset the frame"
        );
    }

    #[test]
    fn test_play_different_animation_resets_to_frame_zero() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);
        player.update(0.15); // advance to frame 1
        assert_eq!(player.current_frame(), 1);

        // Act: switch to a different animation
        player.play(&ANIM_ALT);

        // Assert
        assert_eq!(
            player.current_frame(),
            0,
            "Switching to a different animation should reset to frame 0"
        );
        assert!(
            !player.is_finished(),
            "Switching animation should clear finished flag"
        );
    }

    #[test]
    fn test_play_resets_finished_flag_on_new_animation() {
        // Arrange: finish a one-shot animation
        let mut player = AnimationPlayer::new(&ANIM_ONESHOT);
        player.update(1.0);
        assert!(player.is_finished());

        // Act: switch to a different animation
        player.play(&ANIM_ALT);

        // Assert
        assert!(
            !player.is_finished(),
            "Playing a new animation should clear the finished flag"
        );
    }

    #[test]
    fn test_flipped_state_is_independent_of_animation() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);

        // Act & Assert: default is not flipped
        assert!(!player.is_flipped(), "Default flipped state should be false");

        // Set flipped
        player.set_flipped(true);
        assert!(player.is_flipped(), "Flipped should be true after set_flipped(true)");

        // Advance frames - flipped state should not change
        player.update(0.2);
        assert!(
            player.is_flipped(),
            "Flipped state should persist through frame updates"
        );

        // Switch animation - flipped state should persist
        player.play(&ANIM_ALT);
        assert!(
            player.is_flipped(),
            "Flipped state should persist across animation changes"
        );

        // Unset flipped
        player.set_flipped(false);
        assert!(!player.is_flipped(), "Flipped should be false after set_flipped(false)");
    }

    #[test]
    fn test_incremental_updates_accumulate_correctly() {
        // Arrange
        let mut player = AnimationPlayer::new(&ANIM_LOOPING);

        // Act: advance in small increments that total one frame_duration
        player.update(0.03);
        player.update(0.03);
        player.update(0.04); // total = 0.10 = one frame_duration

        // Assert
        assert_eq!(
            player.current_frame(),
            1,
            "Accumulated small dt values should advance the frame"
        );
    }
}
