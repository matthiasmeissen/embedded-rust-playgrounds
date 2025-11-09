
// Lets us use core libarary, sincd std not suitable for embedded devices that dos not run any os on it
#![no_std]
// Without an os there is no main function that is running
#![no_main]

// Instead we use cortex_m_rt to specify entry point for the program
use cortex_m_rt::entry;
// The microbit crate is a BSC with an abstraction layer to access the chip with sensors
use microbit::board::Board;
use embedded_hal::digital::OutputPin;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut pins = board.display_pins;

    pins.row1.set_low().unwrap();
    pins.col1.set_high().unwrap();

    loop {}
}
