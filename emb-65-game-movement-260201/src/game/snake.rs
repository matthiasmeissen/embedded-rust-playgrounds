use heapless::spsc::Queue;
use crate::game::{coords::Coords, movement::Direction};

#[derive(Debug)]
pub struct Snake {
    pub head: Coords,
    pub tail: Queue<Coords, 32>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let head = Coords::new(2, 2);
        let initial_tail = Coords::new(2, 1);

        let mut tail = Queue::new();
        tail.enqueue(initial_tail).unwrap();
        Self { 
            head, 
            tail, 
            direction: Direction::Right,
        }
    }

    pub fn step_grow(&mut self, coord: Coords) {
        let _ = self.tail.enqueue(coord);
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}
