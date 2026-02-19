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
    pub fn current_sprite(&self) -> &'static SpriteData {
        self.current_animation.frames[self.current_frame]
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
