
pub mod coords;
use rtt_target::{rprintln};

pub fn test() {
    let coords = coords::Coords::new(0, 5);
    rprintln!("Coords {:?}, Out of Bounds: {:?}", coords, coords.is_out_of_bounds());

    let coords = coords::Coords::new(6, -2);
    rprintln!("Coords {:?}, Out of Bounds: {:?}", coords, coords.is_out_of_bounds());

    let coords = coords::Coords::new(1, 4);
    rprintln!("Coords {:?}, Out of Bounds: {:?}", coords, coords.is_out_of_bounds());
}
