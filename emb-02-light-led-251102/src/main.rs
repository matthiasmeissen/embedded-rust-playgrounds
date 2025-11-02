#![deny(unsafe_code)]
#![no_main]
#![no_std]

// No std means there is no standard libarary
// This means no Vec, no String, no println!() and so on

// No main means we do not emit a main symbol from the program
// To define the entry point of the program we import the #[entry] macro from cortex_m_rt crate
// Note that the function is still called main() but could be anything
// Also note that we return ! from it, which means that our program never terminates, it will always keep running

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::board::Board;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    loop {}
}
