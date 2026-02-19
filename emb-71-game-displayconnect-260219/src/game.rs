use microbit::hal::Rng;

use crate::game::coords::Coords;

pub mod coords;
pub mod rng;
pub mod snake;
pub mod movement;


pub struct Game {
    pub status: movement::GameStatus,
    rng: rng::Prng,
    snake: snake::Snake,
    food_coords: coords::Coords,
    speed: u8,
    score: u8,
}

impl Game {
    pub fn new(rng: &mut Rng) -> Self {
        let mut rng = rng::Prng::seeded(rng);
        let snake = snake::Snake::make_snake();
        let food_coords = coords::Coords::random(&mut rng, Some(&snake.coord_set));
        Self { 
            status: movement::GameStatus::Ongoing, 
            rng, 
            snake, 
            food_coords, 
            speed: 1, 
            score: 0 
        }
    }

    pub fn reset(&mut self) {
        self.snake = snake::Snake::make_snake();
        self.place_food();
        self.speed = 1;
        self.status = movement::GameStatus::Ongoing;
    }

    fn place_food(&mut self) -> Coords {
        let coords = coords::Coords::random(&mut self.rng, Some(&self.snake.coord_set));
        self.food_coords = coords;
        coords
    }

    fn wraparound(&self, coords: Coords) -> Coords {
        if coords.row < 0 {
            Coords { row: 4, ..coords }
        } else if coords.row >= 5 {
            Coords { row: 0, ..coords }
        } else if coords.col < 0 {
            Coords { col: 4, ..coords }
        } else {
            Coords { col: 0, ..coords }
        }
    }
    
    pub fn game_matrix(&self, head_brightness: u8, tail_brightness: u8, food_brightness: u8) -> [[u8; 5]; 5] {
        let mut values = [[0; 5]; 5];
        values[self.snake.head.row as usize][self.snake.head.col as usize] = head_brightness;
        for t in &self.snake.tail {
            values[t.row as usize][t.col as usize] = tail_brightness;
        }
        values[self.food_coords.row as usize][self.food_coords.col as usize] = food_brightness;
        values
    }
}

