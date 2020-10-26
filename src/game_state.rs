use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use rand::Rng;
use std::f64;

use crate::models::World;

use crate::geometry::{Advance, Position, Size};
use crate::controllers::CollisionsController;

//use crate::util;
const GRID: f64 = 50.0;

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub world: World,
    current_time: f64,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Size) -> GameState {
        GameState {
            world: World::new(size),
            current_time: 0.0,
        }
    }

        /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update_seconds(state: &mut GameState, dt: f64, num_player: usize) {
        state.current_time += dt;
        // Update rocket rotation

        if state.world.actions[num_player].up {
//            *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
            state.world.player[num_player].vector.direction = 1.5 * f64::consts::PI;
        }
        if state.world.actions[num_player].down {
//            *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
            state.world.player[num_player].vector.direction = 0.5 * f64::consts::PI;
        }
        if state.world.actions[num_player].left {
//            *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
            state.world.player[num_player].vector.direction = f64::consts::PI;
        }
        if state.world.actions[num_player].right {
//            *state.world.player.direction_mut() += ROTATE_SPEED * dt;
            state.world.player[num_player].vector.direction = 0.0;
        };

//        let ptow_collision_flag = CollisionsController::player_walls_collision(state, num_player, GRID);

        // Set speed and advance the player with wrap around
        let mut speed =/* if ptow_collision_flag {
            0.0
        }else */if 
            state.world.actions[num_player].up || 
            state.world.actions[num_player].down || 
            state.world.actions[num_player].left || 
            state.world.actions[num_player].right {
            state.world.player[num_player].speed
        }else {
            0.0
        };

//        let speed:f64 = 0.0;
        state.world.player[num_player].advance_wrapping(dt * speed, state.world.size);
    }
}

