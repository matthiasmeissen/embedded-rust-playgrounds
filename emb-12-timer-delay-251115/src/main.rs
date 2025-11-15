
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{board, gpio};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut pins = board.display_pins;

    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();

    loop {}
}
