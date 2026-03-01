use microbit::hal::Rng;

use crate::game::{coords::Coords, movement::{Direction, StepOutcome, Turn}};

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
    pub score: u8,
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
        self.score = 0;
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

    fn get_next_move(&self) -> Coords {
        let head = self.snake.head;
        let next_move = match self.snake.direction {
            Direction::Up => Coords {
                row: head.row - 1,
                col: head.col
            },
            Direction::Down => Coords {
                row: head.row + 1,
                col: head.col
            },
            Direction::Left => Coords {
                row: head.row,
                col: head.col - 1
            },
            Direction::Right => Coords {
                row: head.row,
                col: head.col + 1
            }
        };
        if next_move.is_out_of_bounds() {
            self.wraparound(next_move)
        } else {
            next_move
        }
    }

    fn get_step_outcome(&self) -> StepOutcome {
        let next_move = self.get_next_move();
        if self.snake.coord_set.contains(&next_move) {
            if next_move != *self.snake.tail.peek().unwrap() {
                StepOutcome::Collision
            } else {
                StepOutcome::Move(next_move)
            }
        } else if next_move == self.food_coords {
            if self.snake.tail.len() == 23 {
                StepOutcome::Full
            } else {
                StepOutcome::Eat(next_move)
            }
        } else {
            StepOutcome::Move(next_move)
        }
    }

    fn handle_step_outcome(&mut self, outcome: StepOutcome) {
        self.status = match outcome {
            StepOutcome::Collision => movement::GameStatus::Lost,
            StepOutcome::Full => movement::GameStatus::Won,
            StepOutcome::Eat(c) => {
                self.snake.move_snake(c, true);
                self.place_food();
                self.score += 1;
                if self.score % 5 == 0 {
                    self.speed += 1;
                }
                movement::GameStatus::Ongoing
            },
            StepOutcome::Move(c) => {
                self.snake.move_snake(c, false);
                movement::GameStatus::Ongoing
            }
        }
    }

    pub fn step(&mut self, turn: Turn) {
        self.snake.turn(turn);
        let outcome = self.get_step_outcome();
        self.handle_step_outcome(outcome);
    }

    pub fn step_len_ms(&self) -> u32 {
        let result = 1000 - (200 * ((self.speed as u32) - 1));
        if result < 200 {
            200 
        } else {
            result as u32
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

    pub fn score_matrix(& self) -> [[u8; 5]; 5] {
        let mut values = [[0; 5]; 5];
        let full_rows = (self.score as usize) / 5;
        for r in 0..full_rows {
            values[r] = [1; 5];
        }
        for c in 0..(self.score as usize) % 5 {
            values[full_rows][c] = 1;
        }
        values
    }
}

