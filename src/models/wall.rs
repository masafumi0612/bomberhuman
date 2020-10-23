//use super::Vector;
//use crate::geometry::{Advance, Collide, Position};
use crate::geometry::{Point};

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Wall {
    pub position: Point,
}

//derive_position_direction!(Player);

impl Wall {
    ///Create a new `Player`
    pub fn new(x: f64, y: f64) -> Wall {
        let new_wall_position = Point::new(x, y);
        Wall {
            position: new_wall_position,
        }
    }
}
