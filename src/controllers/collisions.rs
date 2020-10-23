use crate::game_state::GameState;
use crate::geometry::{Collide, Position};
//use crate::util;

use std::f64;

//const SCORE_PER_ENEMY: u32 = 10;

pub struct CollisionsController;

impl CollisionsController {
    //pub fn handle_collisions(state: &mut GameState, num_player: usize, grid: f64) {
        //CollisionsController::player_wall_collisions(state, num_player, grid);
//        CollisionsController::handle_bullet_collisions(state);
//        CollisionsController::handle_player_collisions(state);
   // }

    // Handles collisions between the bullets and the enemies
    //
    // When an enemy is reached by a bullet, both the enemy and the bullet
    // will be removed. Additionally, the score of the player will be increased.

    pub fn player_walls_collision(state: &mut GameState, num_player: usize, grid: f64) -> bool {
        let mut flag: bool = false;
        for wall in &state.world.wall
        {
            let mut x_distance = state.world.player[num_player].vector.position.x - wall.position.x;
            let mut y_distance = state.world.player[num_player].vector.position.y - wall.position.y;
            let mut distance = (x_distance * x_distance + y_distance * y_distance).sqrt();

            if state.world.actions[num_player].up && distance <= grid/2.0 * 5.0_f64.sqrt() && y_distance > 0.0 && x_distance.abs() < grid / 2.0 {
                flag = true;
/*                if -x_distance <= grid / 2.0 && x_distance <= grid / 2.0 {
                    flag = true;
                    break;
                }else if x_distance > -grid  && x_distance < -grid / 2.0 {
                    state.world.player[num_player].vector.direction = f64::consts::PI;
                }else if x_distance < grid  && x_distance > grid / 2.0{
                    state.world.player[num_player].vector.direction = 0.0;
                }
            }else{
                true
            };
            */
        };
        
        }
        flag
    }

/*
    fn handle_bullet_collisions(state: &mut GameState) {
        let old_enemy_count = state.world.enemies.len();

        // We introduce a scope to shorten the lifetime of the borrows below
        {
            let bullets = &mut state.world.bullets;
            let enemies = &mut state.world.enemies;
            let particles = &mut state.world.particles;

            // Note: this is O(n * m) where n = amount of bullets and n = amount of enemies
            // This is pretty bad, but we don't care because n and m are small
            util::fast_retain(bullets, |bullet| {
                // Remove the first enemy that collides with a bullet (if any)
                // Add an explosion on its place
                if let Some((index, position)) = enemies
                    .iter()
                    .enumerate()
                    .find(|&(_, enemy)| enemy.collides_with(bullet))
                    .map(|(index, enemy)| (index, enemy.position()))
                {
                    util::make_explosion(particles, &position, 10);
                    enemies.remove(index);
                    false
                } else {
                    true
                }
            });
        }

        let killed_enemies = (old_enemy_count - state.world.enemies.len()) as u32;
        state.score += SCORE_PER_ENEMY * killed_enemies;
    }
*/
    // Handles collisions between the player and the enemies

/*
    fn handle_player_collisions(state: &mut GameState) {
        if state
            .world
            .enemies
            .iter()
            .any(|enemy| state.world.player.collides_with(enemy))
        {
            // Make an explosion where the player was
            let ppos = state.world.player.position();
            util::make_explosion(&mut state.world.particles, &ppos, 8);

            state.reset();
        }
    }
*/
}

