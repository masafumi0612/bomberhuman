use crate::geometry::{Point};

#[derive(Default)]
pub struct Pow {
    pub whose: usize,
    pub content: usize, //content type ... 0: bomb_pow, 1: bomb_num, 2: player_speed
    pub position: Point,
}

//derive_position_direction!(Player);

impl Pow {
    pub fn new(content: usize, position: Point) -> Pow {
        Pow {
            whose: 100,
            content: content,
            position: position,
        }
    }
}
