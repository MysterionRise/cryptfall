#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Stores current and previous position for interpolated rendering.
pub struct Transform {
    pub position: Vec2,
    pub prev_position: Vec2,
}

impl Transform {
    pub fn new(x: f32, y: f32) -> Self {
        let pos = Vec2::new(x, y);
        Self {
            position: pos,
            prev_position: pos,
        }
    }

    /// Snapshot current position as the previous position.
    /// Call at the start of each fixed update.
    pub fn commit(&mut self) {
        self.prev_position = self.position;
    }

    /// Linearly interpolate between prev and current position.
    /// `alpha` ranges from 0.0 (prev) to 1.0 (current).
    pub fn interpolated(&self, alpha: f32) -> Vec2 {
        Vec2 {
            x: self.prev_position.x + (self.position.x - self.prev_position.x) * alpha,
            y: self.prev_position.y + (self.position.y - self.prev_position.y) * alpha,
        }
    }
}
