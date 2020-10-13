use rand::Rng;

use crate::geometry::Size;
use crate::models::{Bullet, Enemy, Particle, Player};
use crate::controllers::Actions;

/// A model that contains the other models and renders them
pub struct World {
    pub player: Vec<Player>,
    pub particles: Vec<Particle>,
    pub actions: Vec<Actions>,
//    pub bullets: Vec<Bullet>,
//    pub enemies: Vec<Enemy>,
    pub size: Size,
}

impl World {
    /// Returns a new world of the given size
    pub fn new<R: Rng>(rng: &mut R, size: Size) -> World {
        World {
//            player: Player::random(rng, size),
            player: vec![],
            particles: Vec::with_capacity(1000),
            actions: vec![],
//            bullets: vec![],
//            enemies: vec![],
            size: size,
        }
    }
}
