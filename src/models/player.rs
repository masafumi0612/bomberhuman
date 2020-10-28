use rand::Rng;

use super::Vector;
use crate::geometry::{Advance, Collide, Position};
use crate::geometry::{Point};

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    pub vector: Vector,
    pub speed: f64,
    pub bomb_power: usize,
    pub bomb_num: usize,
}

derive_position_direction!(Player);

impl Player {
    ///Create a new `Player`
    pub fn new(vector: Vector, player_speed: f64, bomb_power: usize, bomb_num: usize) -> Player {
        Player {
            vector: vector, 
            speed: player_speed,
            bomb_power: bomb_power,
            bomb_num: bomb_num,
        }
    }
}

impl Collide for Player {
    fn radius(&self) -> f64 {
        6.0
    }
}
