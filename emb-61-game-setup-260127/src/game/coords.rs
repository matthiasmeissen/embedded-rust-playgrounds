
#[derive(Debug)]
pub struct Coords {
    pub row: i8,
    pub col: i8,
}

impl Coords {
    pub fn new(row: i8, col: i8) -> Self {
        Self {row, col}
    }
}
