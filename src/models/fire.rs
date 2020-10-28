use crate::geometry::{Point};
use std::os::raw::c_double;

#[derive(Default)]
pub struct Fire {
    pub ttl: c_double,
    pub position: Point,
}

//derive_position_direction!(Player);

impl Fire {
    pub fn new(ttl: c_double, position: Point) -> Fire {
        Fire {
            ttl: ttl,
            position: position,
        }
    }
}
