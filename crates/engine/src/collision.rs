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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_overlaps_returns_true_for_overlapping_rects() {
        // Arrange
        let a = AABB::new(0.0, 0.0, 10.0, 10.0);
        let b = AABB::new(5.0, 5.0, 10.0, 10.0);

        // Act
        let result = a.overlaps(&b);

        // Assert
        assert!(result, "Partially overlapping rects should report overlap");
    }

    #[test]
    fn test_aabb_overlaps_returns_false_for_non_overlapping_rects() {
        // Arrange
        let a = AABB::new(0.0, 0.0, 5.0, 5.0);
        let b = AABB::new(20.0, 20.0, 5.0, 5.0);

        // Act
        let result = a.overlaps(&b);

        // Assert
        assert!(!result, "Distant rects should not report overlap");
    }

    #[test]
    fn test_aabb_overlaps_returns_false_for_edge_touching_rects() {
        // Arrange: b starts exactly where a ends (strict < comparison means no overlap)
        let a = AABB::new(0.0, 0.0, 10.0, 10.0);
        let b = AABB::new(10.0, 0.0, 10.0, 10.0); // touches right edge
        let c = AABB::new(0.0, 10.0, 10.0, 10.0); // touches bottom edge

        // Act & Assert
        assert!(
            !a.overlaps(&b),
            "Rects touching on right edge should NOT overlap with strict < comparison"
        );
        assert!(
            !a.overlaps(&c),
            "Rects touching on bottom edge should NOT overlap with strict < comparison"
        );
    }

    #[test]
    fn test_aabb_zero_size_rect_does_not_panic() {
        // Arrange: zero-size rect at a point inside another rect
        let a = AABB::new(5.0, 5.0, 0.0, 0.0);
        let b = AABB::new(0.0, 0.0, 10.0, 10.0);

        // Act & Assert: should not panic regardless of outcome
        // With strict < and >, a zero-size point at (5,5) inside (0,0)-(10,10):
        //   5 < 10 (true), 5+0 > 0 (true), 5 < 10 (true), 5+0 > 0 (true)
        // So it reports overlap (the point is inside the rect).
        let _ = a.overlaps(&b);
        let _ = b.overlaps(&a);
    }

    #[test]
    fn test_aabb_zero_size_rect_at_origin_no_overlap() {
        // Arrange: zero-size rect at origin, separate rect far away
        let a = AABB::new(0.0, 0.0, 0.0, 0.0);
        let b = AABB::new(10.0, 10.0, 5.0, 5.0);

        // Act & Assert
        // 0 < 15 (true), 0+0 > 10 (false) => no overlap
        assert!(
            !a.overlaps(&b),
            "Zero-size rect at origin should not overlap distant rect"
        );
    }

    #[test]
    fn test_aabb_two_zero_size_rects_no_overlap() {
        // Arrange: two zero-size rects at the same point
        let a = AABB::new(5.0, 5.0, 0.0, 0.0);
        let b = AABB::new(5.0, 5.0, 0.0, 0.0);

        // Act & Assert
        // 5 < 5+0 (false) => no overlap
        assert!(
            !a.overlaps(&b),
            "Two zero-size rects at the same point should not overlap (strict < fails on zero width)"
        );
    }

    #[test]
    fn test_aabb_overlaps_with_negative_coordinates() {
        // Arrange
        let a = AABB::new(-10.0, -10.0, 15.0, 15.0); // extends from (-10,-10) to (5,5)
        let b = AABB::new(-3.0, -3.0, 6.0, 6.0); // extends from (-3,-3) to (3,3)

        // Act
        let result = a.overlaps(&b);

        // Assert
        assert!(
            result,
            "Rects with negative coordinates should overlap when they actually intersect"
        );
    }

    #[test]
    fn test_aabb_overlaps_returns_false_for_negative_non_overlapping() {
        // Arrange
        let a = AABB::new(-20.0, -20.0, 5.0, 5.0); // (-20,-20) to (-15,-15)
        let b = AABB::new(10.0, 10.0, 5.0, 5.0); // (10,10) to (15,15)

        // Act & Assert
        assert!(
            !a.overlaps(&b),
            "Distant rects in different quadrants should not overlap"
        );
    }

    #[test]
    fn test_aabb_overlaps_returns_true_for_identical_rects() {
        // Arrange
        let a = AABB::new(3.0, 4.0, 7.0, 8.0);
        let b = AABB::new(3.0, 4.0, 7.0, 8.0);

        // Act
        let result = a.overlaps(&b);

        // Assert
        assert!(result, "Identical rects should overlap");
    }

    #[test]
    fn test_aabb_overlaps_is_symmetric() {
        // Arrange
        let a = AABB::new(0.0, 0.0, 10.0, 10.0);
        let b = AABB::new(5.0, 5.0, 10.0, 10.0);

        // Act & Assert
        assert_eq!(
            a.overlaps(&b),
            b.overlaps(&a),
            "Overlap check should be symmetric"
        );
    }

    #[test]
    fn test_aabb_at_offsets_correctly() {
        // Arrange
        let local = AABB::new(1.0, 2.0, 5.0, 6.0);

        // Act
        let world = local.at(10.0, 20.0);

        // Assert
        assert_eq!(world.x, 11.0, "at() should add px to x");
        assert_eq!(world.y, 22.0, "at() should add py to y");
        assert_eq!(world.w, 5.0, "at() should preserve width");
        assert_eq!(world.h, 6.0, "at() should preserve height");
    }

    #[test]
    fn test_aabb_at_with_negative_offset() {
        // Arrange
        let local = AABB::new(5.0, 5.0, 3.0, 3.0);

        // Act
        let world = local.at(-10.0, -10.0);

        // Assert
        assert_eq!(world.x, -5.0, "at() should work with negative offsets");
        assert_eq!(world.y, -5.0, "at() should work with negative offsets");
    }

    #[test]
    fn test_aabb_center_computes_correctly() {
        // Arrange
        let aabb = AABB::new(0.0, 0.0, 10.0, 20.0);

        // Act
        let (cx, cy) = aabb.center();

        // Assert
        assert_eq!(cx, 5.0, "Center x should be x + w/2");
        assert_eq!(cy, 10.0, "Center y should be y + h/2");
    }

    #[test]
    fn test_aabb_center_with_offset_rect() {
        // Arrange
        let aabb = AABB::new(10.0, 20.0, 6.0, 8.0);

        // Act
        let (cx, cy) = aabb.center();

        // Assert
        assert_eq!(cx, 13.0, "Center x should be 10 + 6/2 = 13");
        assert_eq!(cy, 24.0, "Center y should be 20 + 8/2 = 24");
    }

    #[test]
    fn test_aabb_center_of_zero_size() {
        // Arrange
        let aabb = AABB::new(7.0, 9.0, 0.0, 0.0);

        // Act
        let (cx, cy) = aabb.center();

        // Assert
        assert_eq!(cx, 7.0, "Center of zero-size AABB should be its position");
        assert_eq!(cy, 9.0, "Center of zero-size AABB should be its position");
    }

    #[test]
    fn test_aabb_one_contains_the_other() {
        // Arrange: b is fully inside a
        let a = AABB::new(0.0, 0.0, 20.0, 20.0);
        let b = AABB::new(5.0, 5.0, 3.0, 3.0);

        // Act & Assert
        assert!(a.overlaps(&b), "Containing rect should overlap contained rect");
        assert!(b.overlaps(&a), "Contained rect should overlap containing rect");
    }
}
