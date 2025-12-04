
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{board, display::blocking::Display, hal::Timer};
use rtt_target::{rtt_init_default, rprintln};
use panic_halt as _;

const DISPLAY1: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0]
];

#[entry]
fn main() -> ! {
    rtt_init_default!();
    rprintln!("Program started");

    let board = board::Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    loop {
        display.show(&mut timer, DISPLAY1, 200);
    }
}
