use crate::geometry::{Point};
use std::os::raw::c_double;

#[derive(Default)]
pub struct Bomb {
    pub ttl: c_double,
    pub power: usize,
    pub position: Point,
}

//derive_position_direction!(Player);

impl Bomb {
    pub fn new(ttl: c_double, power: usize, position: Point) -> Bomb {
        Bomb {
            ttl: ttl,
            power: power,
            position: position,
        }
    }
}
