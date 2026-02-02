
use crate::game::coords::Coords;

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

pub enum Turn {
    Left,
    Right,
    None
}

pub enum GameStatus {
    Won,
    Lost,
    Ongoing,
}

pub enum StepOutcome {
    Full,
    Collision,
    Eat(Coords),
    Move(Coords),
}
