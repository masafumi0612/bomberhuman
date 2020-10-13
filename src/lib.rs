extern crate itertools_num;
extern crate pcg_rand;
extern crate rand;

mod controllers;
mod game_state;
mod geometry;
mod models;
mod util;

use pcg_rand::Pcg32Basic;
use rand::SeedableRng;
use std::f64;
use std::os::raw::{c_double, c_int};

use self::models::{Player, Vector};
use self::controllers::{Actions, CollisionsController};
use self::game_state::{GameState, TimeController};
use self::geometry::{Point, Size};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct GameData {
    state: GameState,
//    actions: Actions,
    time_controller: TimeController<Pcg32Basic>,
}

#[wasm_bindgen]
impl GameData {
    pub fn new() -> GameData {
        let draw = Draw::new();
        let width = draw.width();
        let height = draw.height();
        GameData {
            state: GameState::new(Size::new(width, height)),
//            actions: Actions::default(),
            time_controller: TimeController::new(Pcg32Basic::from_seed([42, 42])),
        }
    }

    pub fn create_player(&mut self, x: f64, y: f64) {
        let new_player_position = Point::new(x, y);
        let new_player_vector = Vector::new(new_player_position, 0.0);
        let new_player = Player::new(new_player_vector, 1000.0);
        self.state.world.player.push(new_player);
        self.state.world.actions.push(Actions::default());
    }

    pub fn update(&mut self, time: c_double) {
        let mut i: usize = 0;
        while i < self.state.world.player.len() {
//            let mut actions = self.state.world.actions[i];

            self.time_controller
                .update_seconds(time, &mut self.state, i);
            CollisionsController::handle_collisions(&mut self.state);
            i =  i+ 1;
        }
    }

    pub fn toggle_shoot(&mut self, b: c_int, num_player: usize) {
        //    let data = &mut DATA.lock().unwrap();
        self.state.world.actions[num_player].shoot = int_to_bool(b);
    }

    pub fn toggle_up(&mut self, b: c_int, num_player: usize) {
        //    let data = &mut DATA.lock().unwrap();
        self.state.world.actions[num_player].up = int_to_bool(b);
    }

    pub fn toggle_down(&mut self, b: c_int, num_player: usize) {
        //    let data = &mut DATA.lock().unwrap();
        self.state.world.actions[num_player].down = int_to_bool(b);
    }

    pub fn toggle_left(&mut self, b: c_int, num_player: usize) {
        //    let data = &mut DATA.lock().unwrap();
        self.state.world.actions[num_player].left = int_to_bool(b);
    }

    pub fn toggle_right(&mut self, b: c_int, num_player: usize) {
        //    let data = &mut DATA.lock().unwrap();
        self.state.world.actions[num_player].right = int_to_bool(b);
    }

    pub fn resize(mut self) {
        self = GameData::new();
    }

    pub fn draw(&mut self) {
        use geometry::{Advance, Position};
        //    let data = &mut DATA.lock().unwrap();
        let world = &self.state.world;

        let draw = Draw::new();

        draw.clear_screen();

        for particle in &world.particles {
            draw.draw_particle(particle.x(), particle.y(), 5.0 * particle.ttl);
        }
/*
        for bullet in &world.bullets {
            draw.draw_bullet(bullet.x(), bullet.y());
        }
*/
/*
        for enemy in &world.enemies {
            draw.draw_enemy(enemy.x(), enemy.y());
        }
*/
        for player in &mut self.state.world.player {
            draw.draw_player(player.x(), player.y(), player.direction());
        }
        draw.draw_score(self.state.score as f64);
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

    #[wasm_bindgen(method)]
    pub fn draw_enemy(this: &Draw, _: c_double, _: c_double);

    #[wasm_bindgen(method)]
    pub fn draw_bullet(this: &Draw, _: c_double, _: c_double);

    #[wasm_bindgen(method)]
    pub fn draw_particle(this: &Draw, _: c_double, _: c_double, _: c_double);

    #[wasm_bindgen(method)]
    pub fn draw_score(this: &Draw, _: c_double);
}
