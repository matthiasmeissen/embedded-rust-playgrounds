
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
    Win,
    Loose,
    Ongoing,
}
