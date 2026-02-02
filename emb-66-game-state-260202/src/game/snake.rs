use heapless::{FnvIndexSet, spsc::Queue};
use crate::game::{coords::Coords, movement::{Direction, Turn}};

#[derive(Debug)]
pub struct Snake {
    pub head: Coords,
    pub tail: Queue<Coords, 32>,
    // A set containing all coors the snake is on for quick colission check
    pub coord_set: FnvIndexSet<Coords, 32>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let head = Coords::new(2, 2);
        let initial_tail = Coords::new(2, 1);

        let mut tail = Queue::new();
        tail.enqueue(initial_tail).unwrap();

        let mut coord_set: FnvIndexSet<Coords, 32> = FnvIndexSet::new();
        coord_set.insert(head).unwrap();
        coord_set.insert(initial_tail).unwrap();
        Self { 
            head, 
            tail, 
            coord_set,
            direction: Direction::Right,
        }
    }

    pub fn move_snake(&mut self, coords: Coords, extend: bool) {
        // Place current head inside the tail
        self.tail.enqueue(self.head).unwrap();
        // Set head to new position
        self.head = coords;
        self.coord_set.insert(coords).unwrap();
        if !extend {
            let back = self.tail.dequeue().unwrap();
            self.coord_set.remove(&back);
        }
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

    pub fn turn(&mut self, direction: Turn) {
        match direction {
            Turn::Right => self.turn_right(),
            Turn::Left => self.turn_left(),
            Turn::None => (),
        }
    }
}
