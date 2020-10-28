use crate::geometry::{Point};
use std::os::raw::c_double;

#[derive(Default)]
pub struct SBlock {
    pub position: Point,
}

//derive_position_direction!(Player);

impl SBlock {
    pub fn new(position: Point) -> SBlock {
        SBlock {
            position: position,
        }
    }
}
