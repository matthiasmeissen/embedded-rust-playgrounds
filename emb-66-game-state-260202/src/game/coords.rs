
use heapless::FnvIndexSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coords {
    pub row: i8,
    pub col: i8,
}

impl Coords {
    pub fn new(row: i8, col: i8) -> Self {
        Self {row, col}
    }

    pub fn is_out_of_bounds(&self) -> bool {
        (self.row as u8) >= 5 || (self.col as u8) >= 5
    }
}
