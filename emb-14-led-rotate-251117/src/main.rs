#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut shape: [[u8; 5]; 5] = [[0; 5]; 5];
    let reset_shape = shape.clone();

    let sequence = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (1, 4),
        (2, 4),
        (3, 4),
        (4, 4),
        (4, 3),
        (4, 2),
        (4, 1),
        (4, 0),
        (3, 0),
        (2, 0),
        (1, 0)
    ];

    loop {
        // Iterate through the sequence array of tupels
        for current_led in sequence {
            // The shape array is what will be displayed
            // We will reset it to only contain 0 at the beginning of each iteration
            shape = reset_shape;
            // Then we switch on the led at row and col position defined in the sequence tupel
            shape[current_led.0][current_led.1] = 1;
            // Next we light up teh display based on the shape array (only 0,0 visible in first rum)
            display.show(&mut timer, shape, 200);
            // Then we clear the whole display
            display.clear();
        }
    }
}