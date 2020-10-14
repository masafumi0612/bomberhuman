use rand::Rng;

use crate::geometry::Size;
use crate::models::Player;
use crate::controllers::Actions;

/// A model that contains the other models and renders them
pub struct World {
    pub player: Vec<Player>,
    pub actions: Vec<Actions>,
    pub size: Size,
}

impl World {
    /// Returns a new world of the given size
    pub fn new(size: Size) -> World {
        World {
            player: vec![],
            actions: vec![],
            size: size,
        }
    }
}
