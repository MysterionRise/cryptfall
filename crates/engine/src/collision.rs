/// Axis-Aligned Bounding Box for collision detection.
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl AABB {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Offset this AABB by an entity's world position (local → world space).
    pub fn at(&self, px: f32, py: f32) -> AABB {
        AABB {
            x: self.x + px,
            y: self.y + py,
            w: self.w,
            h: self.h,
        }
    }

    /// Standard AABB overlap test.
    pub fn overlaps(&self, other: &AABB) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }

    /// Center point of the box.
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w * 0.5, self.y + self.h * 0.5)
    }
}
