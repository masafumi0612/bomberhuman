extern crate itertools_num;
extern crate pcg_rand;
extern crate rand;

mod controllers;
mod game_state;
mod geometry;
mod models;
//mod util;

use pcg_rand::Pcg32Basic;
use rand::SeedableRng;
use std::f64;
use std::os::raw::{c_double, c_int};

use self::models::{Player, Wall, Vector};
use self::controllers::{Actions, CollisionsController};
use self::game_state::GameState;
use self::geometry::{Point, Size};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const WORLD_SIZE_WIDTH: f64 = 750.0;
const WORLD_SIZE_HEIGHT: f64 = 650.0;
const PLAYER_SIZE_WIDTH: f64 = 50.0;
const PLAYER_SIZE_HEIGHT: f64 = 50.0;
const WALL_SIZE_WIDTH: f64 = 50.0;
const WALL_SIZE_HEIGHT: f64 = 50.0;
const GRID: f64 = 50.0;

#[wasm_bindgen]
pub struct GameData {
    state: GameState,
}

#[wasm_bindgen]
impl GameData {
    pub fn new() -> GameData {
        let draw = Draw::new();
        let width = draw.width(WORLD_SIZE_WIDTH);
        let height = draw.height(WORLD_SIZE_HEIGHT);
        GameData {
            state: GameState::new(Size::new(width, height)),    
        }
    }

    pub fn create_player(&mut self, x: f64, y: f64) {
        let new_player_position = Point::new(x, y);
        let new_player_vector = Vector::new(new_player_position, 0.0);
        let new_player = Player::new(new_player_vector, 300.0);
        self.state.world.player.push(new_player);
        self.state.world.actions.push(Actions::default());
    }

    pub fn create_wall(&mut self) {
        let mut wall_point_width = WALL_SIZE_WIDTH / 2.0;
        let mut wall_point_height = WALL_SIZE_HEIGHT / 2.0;
        while wall_point_width < WORLD_SIZE_WIDTH {
            let mut new_wall = Wall::new(wall_point_width, wall_point_height);
            self.state.world.wall.push(new_wall);
            wall_point_width+= WALL_SIZE_WIDTH;
        }
        wall_point_width = WALL_SIZE_WIDTH / 2.0;
        wall_point_height = WORLD_SIZE_HEIGHT - WALL_SIZE_HEIGHT / 2.0;
        while wall_point_width < WORLD_SIZE_WIDTH {
            let mut new_wall = Wall::new(wall_point_width, wall_point_height);
            self.state.world.wall.push(new_wall);
            wall_point_width+= WALL_SIZE_WIDTH;
        }
        wall_point_width = WALL_SIZE_WIDTH / 2.0;
        wall_point_height = WALL_SIZE_HEIGHT * 1.5;
        while wall_point_height < WORLD_SIZE_HEIGHT {
            let mut new_wall = Wall::new(wall_point_width, wall_point_height);
            self.state.world.wall.push(new_wall);
            wall_point_height+= WALL_SIZE_HEIGHT;
        }
        wall_point_width = WORLD_SIZE_WIDTH - WALL_SIZE_WIDTH / 2.0;
        wall_point_height = WALL_SIZE_HEIGHT * 1.5;
        while wall_point_height <= WORLD_SIZE_HEIGHT {
            let mut new_wall = Wall::new(wall_point_width, wall_point_height);
            self.state.world.wall.push(new_wall);
            wall_point_height+= WALL_SIZE_HEIGHT;
        }
        wall_point_width = WALL_SIZE_WIDTH * 2.5;
        wall_point_height = WALL_SIZE_HEIGHT * 2.5;
        while wall_point_height < WORLD_SIZE_HEIGHT - WALL_SIZE_HEIGHT * 1.5 {
            while wall_point_width < WORLD_SIZE_WIDTH - WALL_SIZE_WIDTH * 1.5 {
                let mut new_wall = Wall::new(wall_point_width, wall_point_height);
                self.state.world.wall.push(new_wall);
                wall_point_width += WALL_SIZE_WIDTH * 2.0;
            }
            wall_point_width = WALL_SIZE_WIDTH * 2.5;
            wall_point_height += WALL_SIZE_HEIGHT * 2.0;
        }
    }

    pub fn update(&mut self, time: c_double) {
        let mut i: usize = 0;
        // Player update
        let player_len: usize = self.state.world.player.len();
        while i < player_len {
            GameState::update_seconds(&mut self.state, time, i);
            CollisionsController::handle_collisions(&mut self.state, i, GRID);
            i =  i+ 1;
        }
    }

    pub fn toggle_shoot(&mut self, b: c_int, num_player: usize) {
        self.state.world.actions[num_player].shoot = int_to_bool(b);
    }

    pub fn toggle_up(&mut self, b: c_int, num_player: usize) {
        self.state.world.actions[num_player].up = int_to_bool(b);
    }

    pub fn toggle_down(&mut self, b: c_int, num_player: usize) {
        self.state.world.actions[num_player].down = int_to_bool(b);
    }

    pub fn toggle_left(&mut self, b: c_int, num_player: usize) {
        self.state.world.actions[num_player].left = int_to_bool(b);
    }

    pub fn toggle_right(&mut self, b: c_int, num_player: usize) {
        self.state.world.actions[num_player].right = int_to_bool(b);
    }
/*
    pub fn resize(&mut self) {
        GameData::new();
    }
*/
    pub fn draw(&mut self) {
        use geometry::{Advance, Position};

        let draw = Draw::new();

        draw.clear_screen();

        for player in &mut self.state.world.player {
            draw.draw_player(player.x(), player.y(), player.direction());
        }

        for wall in &mut self.state.world.wall {
            draw.draw_wall(wall.position.x, wall.position.y);
        }
/*
        let mut wall_point_width = 25.0;
        let mut wall_point_height = 25.0;
        while wall_point_width < 750.0 {
            draw.draw_wall(wall_point_width, wall_point_height);
            wall_point_width+= 50.0;
        }
        wall_point_width = 25.0;
        wall_point_height = 625.0;
        while wall_point_width < 750.0 {
            draw.draw_wall(wall_point_width, wall_point_height);
            wall_point_width+= 50.0;
        }
        wall_point_width = 25.0;
        wall_point_height = 75.0;
        while wall_point_height < 650.0 {
            draw.draw_wall(wall_point_width, wall_point_height);
            wall_point_height+= 50.0;
        }
        wall_point_width = 725.0;
        wall_point_height = 75.0;
        while wall_point_height <= 650.0 {
            draw.draw_wall(wall_point_width, wall_point_height);
            wall_point_height+= 50.0;
        }
        wall_point_width = 125.0;
        wall_point_height = 125.0;
        while wall_point_height < 575.0 {
            while wall_point_width < 675.0 {
                draw.draw_wall(wall_point_width, wall_point_height);
                wall_point_width += 100.0;
            }
            wall_point_width = 125.0;
            wall_point_height += 100.0;
        }
*/

    }
}

fn int_to_bool(i: c_int) -> bool {
    i != 0
}

// These functions are provided by the runtime
#[wasm_bindgen(module = "/src/javascript/draw.js")]
extern "C" {
    type Draw;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Draw;

    #[wasm_bindgen(method)]
    pub fn width(this: &Draw, x: f64) -> f64;

    #[wasm_bindgen(method)]
    pub fn height(this: &Draw, y: f64) -> f64;

    #[wasm_bindgen(method)]
    pub fn clear_screen(this: &Draw);

    #[wasm_bindgen(method)]
    pub fn draw_player(this: &Draw, _: c_double, _: c_double, _: c_double);

    #[wasm_bindgen(method)]
    pub fn draw_wall(this: &Draw, _: c_double, _: c_double);

}
