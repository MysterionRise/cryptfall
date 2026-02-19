/// Camera that follows a target with smooth lerp and screen shake.
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub viewport_w: usize,
    pub viewport_h: usize,
    pub smoothing: f32, // 0.0 = instant snap, 0.95 = very smooth lag

    shake_intensity: f32,
    shake_decay: f32,
    shake_offset_x: f32,
    shake_offset_y: f32,
    rng_state: u32,
}

impl Camera {
    pub fn new(viewport_w: usize, viewport_h: usize) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            viewport_w,
            viewport_h,
            smoothing: 0.85,
            shake_intensity: 0.0,
            shake_decay: 0.85,
            shake_offset_x: 0.0,
            shake_offset_y: 0.0,
            rng_state: 48271,
        }
    }

    /// Set the target to follow (usually player center in world pixels).
    pub fn follow(&mut self, world_x: f32, world_y: f32) {
        self.target_x = world_x - self.viewport_w as f32 / 2.0;
        self.target_y = world_y - self.viewport_h as f32 / 2.0;
    }

    /// Update camera position. Frame-rate independent smoothing.
    pub fn update(&mut self, dt: f64) {
        let t = 1.0 - self.smoothing.powf(dt as f32 * 30.0);
        self.x += (self.target_x - self.x) * t;
        self.y += (self.target_y - self.y) * t;

        // Update shake
        if self.shake_intensity > 0.1 {
            self.shake_offset_x = self.rand_signed() * self.shake_intensity;
            self.shake_offset_y = self.rand_signed() * self.shake_intensity;
            self.shake_intensity *= self.shake_decay;
        } else {
            self.shake_offset_x = 0.0;
            self.shake_offset_y = 0.0;
            self.shake_intensity = 0.0;
        }
    }

    /// Clamp camera to world bounds (don't show outside the map).
    pub fn clamp_to_bounds(&mut self, world_w: f32, world_h: f32) {
        self.x = self
            .x
            .clamp(0.0, (world_w - self.viewport_w as f32).max(0.0));
        self.y = self
            .y
            .clamp(0.0, (world_h - self.viewport_h as f32).max(0.0));
    }

    /// Get the final camera offset (position + shake) for rendering.
    pub fn offset(&self) -> (i32, i32) {
        (
            (self.x + self.shake_offset_x) as i32,
            (self.y + self.shake_offset_y) as i32,
        )
    }

    /// Trigger screen shake with the given intensity.
    pub fn shake(&mut self, intensity: f32) {
        self.shake_intensity = intensity;
    }

    /// Convert world coordinates to screen coordinates.
    pub fn world_to_screen(&self, wx: f32, wy: f32) -> (i32, i32) {
        let (ox, oy) = self.offset();
        (wx as i32 - ox, wy as i32 - oy)
    }

    /// Snap camera position to target immediately (no lerp).
    pub fn snap(&mut self) {
        self.x = self.target_x;
        self.y = self.target_y;
    }

    /// Simple LCG pseudo-random returning [-0.5, 0.5].
    fn rand_signed(&mut self) -> f32 {
        self.rng_state = self.rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.rng_state >> 16) as f32 / 65535.0 - 0.5
    }
}
