use crate::game_state::GameState;
use crate::models::{Bomb, SBlock, Pow};
use crate::geometry::{Collide, Position};
//use crate::util;

use std::f64;

//const SCORE_PER_ENEMY: u32 = 10;

pub struct CollisionsController;

impl CollisionsController {
    pub fn player_collisions(state: &mut GameState, num_player: usize, grid: f64) {
        CollisionsController::player_walls_collision(state, num_player, grid);
        CollisionsController::player_sblocks_collision(state, num_player, grid);
        CollisionsController::player_pows_collision(state, num_player, grid);
        CollisionsController::player_bombs_collision(state, num_player, grid);
        CollisionsController::player_fires_collision(state, num_player, grid);
//        CollisionsController::handle_bullet_collisions(state);
//        CollisionsController::handle_player_collisions(state);
    }

    // Handles collisions between the bullets and the enemies
    //
    // When an enemy is reached by a bullet, both the enemy and the bullet
    // will be removed. Additionally, the score of the player will be increased.

    pub fn player_walls_collision(state: &mut GameState, num_player: usize, grid: f64){
        let mut flag: bool = false;
        for wall in &state.world.wall
        {
            let mut x_distance = state.world.player[num_player].vector.position.x - wall.position.x;
            let mut y_distance = state.world.player[num_player].vector.position.y - wall.position.y;
            if x_distance < grid && x_distance >= 0.0 && state.world.actions[num_player].left {
                if y_distance.abs() <= grid / 2.0 {
                    state.world.player[num_player].vector.position.x = wall.position.x + grid;
                }else if grid / 2.0 < y_distance && y_distance < grid{
                    state.world.player[num_player].vector.position.x = wall.position.x + grid;
                    state.world.player[num_player].vector.position.y -= x_distance - grid;
                    if state.world.player[num_player].vector.position.y > wall.position.y + grid {
                        state.world.player[num_player].vector.position.y = wall.position.y + grid;
                    }
                }else if grid / 2.0 < -y_distance && -y_distance < grid{
                    state.world.player[num_player].vector.position.x = wall.position.x + grid;
                    state.world.player[num_player].vector.position.y += x_distance - grid;
                    if state.world.player[num_player].vector.position.y < wall.position.y - grid {
                        state.world.player[num_player].vector.position.y = wall.position.y - grid;
                    }                
                } 
            }else if x_distance < 0.0 && x_distance > -grid  && state.world.actions[num_player].right {
                if y_distance.abs() <= grid / 2.0 {
                    state.world.player[num_player].vector.position.x = wall.position.x - grid;
                }else if grid / 2.0 < y_distance && y_distance < grid {
                    state.world.player[num_player].vector.position.x = wall.position.x - grid;
                    state.world.player[num_player].vector.position.y += x_distance + grid;
                    if state.world.player[num_player].vector.position.y > wall.position.y + grid {
                        state.world.player[num_player].vector.position.y = wall.position.y + grid;
                    }
                }else if grid / 2.0 < -y_distance && -y_distance < grid {
                    state.world.player[num_player].vector.position.x = wall.position.x - grid;
                    state.world.player[num_player].vector.position.y -= x_distance + grid;
                    if state.world.player[num_player].vector.position.y < wall.position.y - grid {
                        state.world.player[num_player].vector.position.y = wall.position.y - grid;
                    }                
                }
            }else if y_distance < grid && y_distance >= 0.0 && state.world.actions[num_player].up {
                if x_distance.abs() <= grid / 2.0 {
                    state.world.player[num_player].vector.position.y = wall.position.y + grid;
                }else if grid / 2.0 < x_distance && x_distance < grid {
                    state.world.player[num_player].vector.position.y = wall.position.y + grid;
                    state.world.player[num_player].vector.position.x -= y_distance - grid;
                    if state.world.player[num_player].vector.position.x > wall.position.x + grid {
                        state.world.player[num_player].vector.position.x = wall.position.x + grid;
                    }
                }else if grid / 2.0 < -x_distance && -x_distance < grid{
                    state.world.player[num_player].vector.position.y = wall.position.y + grid;
                    state.world.player[num_player].vector.position.x += y_distance - grid;
                    if state.world.player[num_player].vector.position.x < wall.position.x - grid {
                        state.world.player[num_player].vector.position.x = wall.position.x - grid;
                    }                
                }
            }else if y_distance < 0.0 && y_distance > -grid && state.world.actions[num_player].down {
                if x_distance.abs() <= grid / 2.0 {
                    state.world.player[num_player].vector.position.y = wall.position.y - grid;
                }else if grid / 2.0 < x_distance && x_distance < grid {
                    state.world.player[num_player].vector.position.y = wall.position.y - grid;
                    state.world.player[num_player].vector.position.x += y_distance + grid;
                    if state.world.player[num_player].vector.position.x > wall.position.x + grid {
                        state.world.player[num_player].vector.position.x = wall.position.x + grid;
                    }
                }else if grid / 2.0 < -x_distance && -x_distance < grid {
                    state.world.player[num_player].vector.position.y = wall.position.y - grid;
                    state.world.player[num_player].vector.position.x -= y_distance + grid;
                    if state.world.player[num_player].vector.position.x < wall.position.x - grid {
                        state.world.player[num_player].vector.position.x = wall.position.x - grid;
                    }      
                }
            }
        }
    }

    pub fn player_sblocks_collision(state: &mut GameState, num_player: usize, grid: f64){
        for sblock in &state.world.sblock {
            let mut x_distance = state.world.player[num_player].vector.position.x - sblock.position.x;
            let mut y_distance = state.world.player[num_player].vector.position.y - sblock.position.y;
            if 0.0 < x_distance && x_distance < grid && y_distance.abs() < grid / 2.0 {
                state.world.player[num_player].vector.position.x = sblock.position.x + grid;
            }else if grid > -x_distance && -x_distance > 0.0 && y_distance.abs() < grid / 2.0 {
                state.world.player[num_player].vector.position.x = sblock.position.x - grid;
            }else if 0.0 < y_distance && y_distance < grid && x_distance.abs() < grid / 2.0 {
                state.world.player[num_player].vector.position.y = sblock.position.y + grid;
            }else if grid > -y_distance && -y_distance > 0.0 && x_distance.abs() < grid / 2.0 {
                state.world.player[num_player].vector.position.y = sblock.position.y - grid;
            }
        }
    }

    pub fn player_pows_collision(state: &mut GameState, num_player: usize, grid: f64){
        for pow in &mut state.world.pow {
            let mut x_distance = state.world.player[num_player].vector.position.x - pow.position.x;
            let mut y_distance = state.world.player[num_player].vector.position.y - pow.position.y;
            if x_distance.abs() < grid / 2.0 && y_distance.abs() < grid / 2.0 && pow.whose == 100 && state.world.player[num_player].alive {
                pow.whose = num_player;
                match pow.content {
                    0 => state.world.player[num_player].bomb_power += 1,
                    1 => state.world.player[num_player].bomb_num += 1,
                    2 => {
                        state.world.player[num_player].speed += 50.0; 
                        if state.world.player[num_player].speed >= 300.0 {
                            state.world.player[num_player].speed = 250.0;
                        }
                    },
                    _ => println!(),
                }
            }
        }
    }

    pub fn player_bombs_collision(state: &mut GameState, num_player: usize, grid: f64){
        for bombs in &state.world.bomb {
            for bomb in bombs {
                let mut x_distance = state.world.player[num_player].vector.position.x - bomb.position.x;
                let mut y_distance = state.world.player[num_player].vector.position.y - bomb.position.y;
                if grid - state.world.player[num_player].speed / 20.0 < x_distance && x_distance < grid && y_distance.abs() < grid / 2.0 && state.world.actions[num_player].left {
                    state.world.player[num_player].vector.position.x = bomb.position.x + grid;
                }else if grid > -x_distance && -x_distance > grid - state.world.player[num_player].speed / 20.0 && y_distance.abs() < grid / 2.0 && state.world.actions[num_player].right {
                    state.world.player[num_player].vector.position.x = bomb.position.x - grid;
                }else if grid - state.world.player[num_player].speed / 20.0 < y_distance && y_distance < grid && x_distance.abs() < grid / 2.0 && state.world.actions[num_player].up {
                    state.world.player[num_player].vector.position.y = bomb.position.y + grid;
                }else if grid > -y_distance && -y_distance > grid - state.world.player[num_player].speed / 20.0 && x_distance.abs() < grid / 2.0 && state.world.actions[num_player].down {
                    state.world.player[num_player].vector.position.y = bomb.position.y - grid;
                }
            }
        }
    }

    pub fn player_fires_collision(state: &mut GameState, num_player: usize, grid: f64) {
        for fire in &state.world.fire {
            let x_distance = state.world.player[num_player].vector.position.x - fire.position.x;
            let y_distance = state.world.player[num_player].vector.position.y - fire.position.y;
            if x_distance.abs() < grid / 1.5 && y_distance.abs() < grid / 2.0 {
                state.world.player[num_player].alive = false;
                break;
            }
            if y_distance.abs() < grid / 1.5 && x_distance.abs() < grid / 2.0 {
                state.world.player[num_player].alive = false;
                break;
            }            
        }
        if state.world.player[num_player].alive == false {
            for pow in &mut state.world.pow {
                if pow.whose == num_player {
                    pow.whose = 100;
                }
            }
        }
    }

    pub fn fire_walls_collision(state: &GameState, fire_position_x: f64, fire_position_y: f64)-> bool {
        let mut fire_wall_collision_flag: bool = false;
        for wall in &state.world.wall {
            if fire_position_x == wall.position.x && fire_position_y == wall.position.y {
                fire_wall_collision_flag = true;
                break;
            }
        }
        fire_wall_collision_flag
    }

    pub fn fire_sblocks_collision(sblock: &mut Vec<SBlock>, fire_position_x: f64, fire_position_y: f64) -> bool {
        let mut i: usize = 0;
        let mut collision_flag: bool = false;
        while i < sblock.len() {
            if fire_position_x == sblock[i].position.x && fire_position_y == sblock[i].position.y {
                &sblock.remove(i);
                collision_flag = true;
                break;
            }
            i+= 1;
        }
        collision_flag
    }

    pub fn fire_pows_collision(pow: &mut Vec<Pow>, fire_position_x: f64, fire_position_y: f64) -> bool {
        let mut i: usize = 0;
        let mut collision_flag: bool = false;
        while i < pow.len() {
            if fire_position_x == pow[i].position.x && fire_position_y == pow[i].position.y && pow[i].whose == 100{
                &pow.remove(i);
                collision_flag = true;
                break;
            }
            i+= 1;
        }
        collision_flag
    }
  
    pub fn fire_bomb_collision(all_bombs: &mut Vec<Vec<Bomb>>, num_player: usize, i: usize, x: f64, y: f64, grid: f64) -> bool {
        let mut collision_flag: bool = false;
        let fire_position_x = all_bombs[num_player][i].position.x + grid * x;
        let fire_position_y = all_bombs[num_player][i].position.y + grid * y;
        for bombs in all_bombs {
            for bomb in bombs {
                if fire_position_x == bomb.position.x && fire_position_y == bomb.position.y {
                    bomb.ttl = 0.00000000001;
                    collision_flag = true;
                    break;
                }
            }
        }
        collision_flag
    }
/*
    pub fn bomb_fire_collision(state: &mut GameState) {
        for bombs in &mut state.world.bomb {
            for bomb in bombs {
                for fire in &state.world.fire {
                    if bomb.position.x == fire.position.x && bomb.position.y == fire.position.y {
                        bomb.ttl = 0.000000000001;
                    }
                }
            }
        }
    }
    */
}