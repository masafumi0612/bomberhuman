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

use self::models::{Player, Vector};
use self::controllers::{Actions, CollisionsController};
use self::game_state::GameState;
use self::geometry::{Point, Size};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct GameData {
    state: GameState,
}

#[wasm_bindgen]
impl GameData {
    pub fn new() -> GameData {
        let draw = Draw::new();
        let width = draw.width();
        let height = draw.height();
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

    pub fn update(&mut self, time: c_double) {
        let mut i: usize = 0;
        while i < self.state.world.player.len() {

            GameState::update_seconds(&mut self.state, time, i);
            CollisionsController::handle_collisions(&mut self.state);
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

    pub fn resize(&mut self) {
        GameData::new();
    }

    pub fn draw(&mut self) {
        use geometry::{Advance, Position};

        let draw = Draw::new();

        draw.clear_screen();

        for player in &mut self.state.world.player {
            draw.draw_player(player.x(), player.y(), player.direction());
        }
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
    pub fn width(this: &Draw) -> f64;

    #[wasm_bindgen(method)]
    pub fn height(this: &Draw) -> f64;

    #[wasm_bindgen(method)]
    pub fn clear_screen(this: &Draw);

    #[wasm_bindgen(method)]
    pub fn draw_player(this: &Draw, _: c_double, _: c_double, _: c_double);
}
