extern crate itertools_num;
extern crate pcg_rand;
extern crate rand;

mod controllers;
mod game_state;
mod geometry;
mod models;
//mod util;

use pcg_rand::Pcg32Basic;
//use rand::SeedableRng;
use rand::Rng;
use std::f64;
use std::os::raw::{c_double, c_int};

use self::models::{Bomb, Fire, Player, Pow, SBlock, Wall, Vector};
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
const PLAYER_SIZE_SPEED: f64 = 100.0;
const BOMB_TTL: c_double = 3.0;
const BOMB_POWER: usize = 1;
const BOMB_NUM: usize = 1; 
const FIRE_TTL: c_double = 1.0;
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
        let new_player_alive: bool = true;
        let new_player_position = Point::new(x, y);
        let new_player_vector = Vector::new(new_player_position, 0.0);
        let new_player_speed = PLAYER_SIZE_SPEED;
        let new_player_bomb_power: usize = BOMB_POWER;
        let new_player_bomb_num: usize = BOMB_NUM;

        let new_player = Player::new(new_player_alive, new_player_vector, new_player_speed, new_player_bomb_power, new_player_bomb_num);
        self.state.world.player.push(new_player);
        self.state.world.bomb.push(Vec::new());
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

    pub fn create_sblock(&mut self) {
        
        let mut new_sblock_position_x = 175.0;
        let mut new_sblock_position_y = 125.0;
        while new_sblock_position_y <= 575.0{
            while new_sblock_position_x <= 600.0 {
                let new_sblock_position = Point::new(new_sblock_position_x, new_sblock_position_y);
                let new_sblock = SBlock::new(new_sblock_position);
                let secret_number = rand::thread_rng().gen_range(0, 100);
                if secret_number <= 50 {
                    self.state.world.sblock.push(new_sblock);
                }
                new_sblock_position_x += GRID * 2.0;
            }
            new_sblock_position_y += GRID * 2.0;
            new_sblock_position_x = 175.0;
        }

        new_sblock_position_x = 225.0;
        new_sblock_position_y = 75.0;
        while new_sblock_position_y <= 575.0{
            while new_sblock_position_x <= 550.0 {
                let new_sblock_position = Point::new(new_sblock_position_x, new_sblock_position_y);
                let new_sblock = SBlock::new(new_sblock_position);
                let secret_number = rand::thread_rng().gen_range(0, 100);
                if secret_number <= 50 {
                    self.state.world.sblock.push(new_sblock);
                }
                new_sblock_position_x += GRID;
            }
            new_sblock_position_y += GRID * 2.0;
            new_sblock_position_x = 225.0;
        }
      
        new_sblock_position_x = 125.0;
        new_sblock_position_y = 175.0;
        while new_sblock_position_y <= 475.0{
            while new_sblock_position_x <= 175.0 {
                let new_sblock_position = Point::new(new_sblock_position_x, new_sblock_position_y);
                let new_sblock = SBlock::new(new_sblock_position);
                let secret_number = rand::thread_rng().gen_range(0, 100);
                if secret_number <= 50 {
                    self.state.world.sblock.push(new_sblock);
                }
                new_sblock_position_x += GRID;
            }
            new_sblock_position_y += GRID * 2.0;
            new_sblock_position_x = 125.0;
        }
      
        new_sblock_position_x = 575.0;
        new_sblock_position_y = 175.0;
        while new_sblock_position_y <= 475.0{
            while new_sblock_position_x <= 625.0 {
                let new_sblock_position = Point::new(new_sblock_position_x, new_sblock_position_y);
                let new_sblock = SBlock::new(new_sblock_position);
                let secret_number = rand::thread_rng().gen_range(0, 100);
                if secret_number <= 50 {
                    self.state.world.sblock.push(new_sblock);
                }
                new_sblock_position_x += GRID;
            }
            new_sblock_position_y += GRID * 2.0;
            new_sblock_position_x = 575.0;
        }

        new_sblock_position_x = 75.0;
        new_sblock_position_y = 225.0;
        while new_sblock_position_y <= 425.0{
            let new_sblock_position = Point::new(new_sblock_position_x, new_sblock_position_y);
            let new_sblock = SBlock::new(new_sblock_position);
            let secret_number = rand::thread_rng().gen_range(0, 100);
            if secret_number <= 50 {
            self.state.world.sblock.push(new_sblock);
            }
            new_sblock_position_y += GRID;
        }

        new_sblock_position_x = 675.0;
        new_sblock_position_y = 225.0;
        while new_sblock_position_y <= 425.0{
            let new_sblock_position = Point::new(new_sblock_position_x, new_sblock_position_y);
            let new_sblock = SBlock::new(new_sblock_position);
            let secret_number = rand::thread_rng().gen_range(0, 100);
            if secret_number <= 50 {
            self.state.world.sblock.push(new_sblock);
            }
            new_sblock_position_y += GRID;
        }

        let new_sblock_position = Point::new(175.0, 75.0);
        let new_sblock = SBlock::new(new_sblock_position);
        self.state.world.sblock.push(new_sblock);
        
        let new_sblock_position = Point::new(75.0, 175.0);
        let new_sblock = SBlock::new(new_sblock_position);
        self.state.world.sblock.push(new_sblock);

        let new_sblock_position = Point::new(575.0, 75.0);
        let new_sblock = SBlock::new(new_sblock_position);
        self.state.world.sblock.push(new_sblock);

        let new_sblock_position = Point::new(675.0, 175.0);
        let new_sblock = SBlock::new(new_sblock_position);
        self.state.world.sblock.push(new_sblock);

        let new_sblock_position = Point::new(75.0, 475.0);
        let new_sblock = SBlock::new(new_sblock_position);
        self.state.world.sblock.push(new_sblock);

        let new_sblock_position = Point::new(175.0, 575.0);
        let new_sblock = SBlock::new(new_sblock_position);
        self.state.world.sblock.push(new_sblock);

        let new_sblock_position = Point::new(675.0, 475.0);
        let new_sblock = SBlock::new(new_sblock_position);
        self.state.world.sblock.push(new_sblock);

        let new_sblock_position = Point::new(575.0, 575.0);
        let new_sblock = SBlock::new(new_sblock_position);
        self.state.world.sblock.push(new_sblock);
    }

    pub fn create_pow(&mut self) {
        for i in 0..self.state.world.sblock.len() {
            let secret_number = rand::thread_rng().gen_range(0, 100);
            if secret_number <= 50 {
                let new_pow_content = rand::thread_rng().gen_range(0, 3);
                let new_pow_position = Point::new(self.state.world.sblock[i].position.x, self.state.world.sblock[i].position.y);
                let new_pow = Pow::new(new_pow_content, new_pow_position);
                self.state.world.pow.push(new_pow);
            }
        }
    }

    pub fn check_bomb_exist(&mut self, x: f64, y: f64) -> bool {
        let mut exist_flag: bool = false;
        for bombs in &self.state.world.bomb {
            for bomb in bombs {
                if x == bomb.position.x && y == bomb.position.y {
                    exist_flag = true;
                }
            }
        }
        exist_flag
    }

    pub fn create_bomb(&mut self, num_player: usize) {
        if self.state.world.player[num_player].alive {
            if self.state.world.actions[num_player].put_bomb && self.state.world.player[num_player].bomb_num > self.state.world.bomb[num_player].len() {
                let new_bomb_position_x = 
                    self.state.world.player[num_player].vector.position.x
                    - self.state.world.player[num_player].vector.position.x % GRID 
                    + GRID / 2.0;
                let new_bomb_position_y = 
                    self.state.world.player[num_player].vector.position.y
                    - self.state.world.player[num_player].vector.position.y % GRID
                        + GRID / 2.0;
                if self.check_bomb_exist(new_bomb_position_x, new_bomb_position_y) == true {
                    return;
                }
                let new_bomb_position = Point::new(new_bomb_position_x, new_bomb_position_y);
                let new_bomb_power = self.state.world.player[num_player].bomb_power;
                let new_bomb_ttl: c_double = BOMB_TTL;
                let new_bomb = Bomb::new(new_bomb_ttl, new_bomb_power, new_bomb_position);
                self.state.world.bomb[num_player].push(new_bomb);
            }
        }
    }

    pub fn create_fire(&mut self, num_player: usize) {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        for bomb in &self.state.world.bomb[num_player] {
            if bomb.ttl <= 0.0 {
                x = -1.0;
                while -x <= bomb.power as f64 {
                    if CollisionsController::fire_walls_collision(&self.state, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    if CollisionsController::fire_sblocks_collision(&mut self.state.world.sblock, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    if CollisionsController::fire_pows_collision(&mut self.state.world.pow, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    let new_fire_position = Point::new(bomb.position.x + GRID * x, bomb.position.y + GRID * y);
                    let new_fire_ttl: c_double = FIRE_TTL;
                    let new_fire = Fire::new(new_fire_ttl, new_fire_position);
                    self.state.world.fire.push(new_fire);
                    x -= 1.0;
                }
                x = 1.0;
                while x <= bomb.power as f64 {
                    if CollisionsController::fire_walls_collision(&self.state, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    if CollisionsController::fire_sblocks_collision(&mut self.state.world.sblock, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    if CollisionsController::fire_pows_collision(&mut self.state.world.pow, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    let new_fire_position = Point::new(bomb.position.x + GRID * x, bomb.position.y + GRID * y);
                    let new_fire_ttl: c_double = FIRE_TTL;
                    let new_fire = Fire::new(new_fire_ttl, new_fire_position);
                    self.state.world.fire.push(new_fire);
                    x += 1.0;
                }
                x = 0.0;
                y = -1.0;
                while -y <= bomb.power as f64 {
                    if CollisionsController::fire_walls_collision(&self.state, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    if CollisionsController::fire_sblocks_collision(&mut self.state.world.sblock, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    if CollisionsController::fire_pows_collision(&mut self.state.world.pow, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    let new_fire_position = Point::new(bomb.position.x + GRID * x, bomb.position.y + GRID * y);
                    let new_fire_ttl: c_double = FIRE_TTL;
                    let new_fire = Fire::new(new_fire_ttl, new_fire_position);
                    self.state.world.fire.push(new_fire);
                    y -= 1.0;
                }
                y = 1.0;
                while y <= bomb.power as f64{
                    if CollisionsController::fire_walls_collision(&self.state, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    if CollisionsController::fire_sblocks_collision(&mut self.state.world.sblock, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    if CollisionsController::fire_pows_collision(&mut self.state.world.pow, bomb.position.x + GRID * x, bomb.position.y + GRID * y) {
                        break;
                    }
                    let new_fire_position = Point::new(bomb.position.x + GRID * x, bomb.position.y + GRID * y);
                    let new_fire_ttl: c_double = FIRE_TTL;
                    let new_fire = Fire::new(new_fire_ttl, new_fire_position);
                    self.state.world.fire.push(new_fire);
                    y += 1.0;
                }
                let new_fire_position = Point::new(bomb.position.x, bomb.position.y);
                let new_fire_ttl: c_double = FIRE_TTL;
                let new_fire = Fire::new(new_fire_ttl, new_fire_position);
                self.state.world.fire.push(new_fire);
            }
        }
    }

    pub fn update(&mut self, time: c_double) {
        let mut i: usize = 0;
        let player_len: usize = self.state.world.player.len();
        while i < player_len {
            GameState::move_player(&mut self.state, time, i);
            GameState::update_bomb_ttl(&mut self.state.world.bomb[i], time);
            GameState::update_fire_ttl(&mut self.state.world.fire, time);
            self.create_bomb(i);
            self.create_fire(i);
            CollisionsController::player_collisions(&mut self.state, i, GRID);
            GameState::delete_bomb(&mut self.state.world.bomb[i]);           
            GameState::delete_fire(&mut self.state.world.fire);
            CollisionsController::bomb_fire_collision(&mut self.state);
            i =  i + 1;
        }
    }

    pub fn put_bomb(&mut self, b: c_int, num_player: usize) {
        self.state.world.actions[num_player].put_bomb = int_to_bool(b);
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

        for wall in &self.state.world.wall {
            draw.draw_wall(wall.position.x, wall.position.y);
        }
   
        for pow in &self.state.world.pow {
            if pow.whose == 100 {
                draw.draw_pow(pow.position.x, pow.position.y, pow.content);
            }
        }

        for sblock in &self.state.world.sblock {
            draw.draw_sblock(sblock.position.x, sblock.position.y);
        }

        for bombs in &self.state.world.bomb {
            for bomb in bombs {
                draw.draw_bomb(bomb.position.x, bomb.position.y);
            }
        }

        for fire in &self.state.world.fire {
            draw.draw_fire(fire.position.x, fire.position.y);
        }

        for player in &self.state.world.player {
            draw.draw_player(player.x(), player.y(), player.direction(), player.alive);
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
    pub fn width(this: &Draw, x: f64) -> f64;

    #[wasm_bindgen(method)]
    pub fn height(this: &Draw, y: f64) -> f64;

    #[wasm_bindgen(method)]
    pub fn clear_screen(this: &Draw);

    #[wasm_bindgen(method)]
    pub fn draw_player(this: &Draw, _: c_double, _: c_double, _: c_double, _: bool);

    #[wasm_bindgen(method)]
    pub fn draw_wall(this: &Draw, _: c_double, _: c_double);

    #[wasm_bindgen(method)]
    pub fn draw_bomb(this: &Draw, _: c_double, _: c_double);

    #[wasm_bindgen(method)]
    pub fn draw_fire(this: &Draw, _: c_double, _: c_double);

    #[wasm_bindgen(method)]
    pub fn draw_pow(this: &Draw, _: c_double, _: c_double, _: usize);

    #[wasm_bindgen(method)]
    pub fn draw_sblock(this: &Draw, _: c_double, _: c_double);
}
