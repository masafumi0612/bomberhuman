use rand::Rng;

use super::Point;

/// A `Size` represents a region in space
#[derive(Clone, Copy, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64
}

impl Size {
    /// Returns a new `Size` of the given dimensions
    pub fn new(width: f64, height: f64) -> Size {
        Size { width: width, height: height }
    }
}
