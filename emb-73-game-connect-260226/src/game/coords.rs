
use heapless::FnvIndexSet;
use rtt_target::{rprint, rprintln};
use super::rng::Prng;
use crate::game::snake::Snake;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coords {
    pub row: i8,
    pub col: i8,
}

impl Coords {
    pub fn new(row: i8, col: i8) -> Self {
        Self {row, col}
    }

    pub fn random(rng: &mut Prng, exclude: Option<&FnvIndexSet<Coords, 32>>) -> Self {
        let mut coords = Coords {
            row: ((rng.random_u32() as usize) % 5) as i8,
            col: ((rng.random_u32() as usize) % 5) as i8,
        };
        while exclude.is_some_and(|exc| exc.contains(&coords)) {
            coords = Coords {
                row: ((rng.random_u32() as usize) % 5) as i8,
                col: ((rng.random_u32() as usize) % 5) as i8,
            }
        }
        coords
    }

    pub fn is_self_colliding(&self, coord_set: &FnvIndexSet<Coords, 32>) -> bool {
        if coord_set.contains(self) {
            true
        } else {
            false
        }
    }

    pub fn is_out_of_bounds(&self) -> bool {
        (self.row as u8) >= 5 || (self.col as u8) >= 5
    }
}
