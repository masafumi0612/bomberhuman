use rand::Rng;

use super::Vector;
use crate::geometry::{Advance, Collide, Position};
use crate::geometry::{Point};

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    pub vector: Vector,
    pub speed: f64,
}

derive_position_direction!(Player);

/// The player is represented as the polygon below
pub const POLYGON: &'static [[f64; 2]] = &[[0.0, -8.0], [20.0, 0.0], [0.0, 8.0]];

impl Player {
    ///Create a new `Player`
    pub fn new(vector: Vector, player_speed: f64) -> Player {
        Player {
            vector: vector, 
            speed: player_speed,
        }
    }
}

impl Collide for Player {
    fn radius(&self) -> f64 {
        6.0
    }
}
