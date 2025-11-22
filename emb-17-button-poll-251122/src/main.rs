
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};
use microbit::{board, display::blocking::Display, hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let lights1 = [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
    ];

    let lights2 = [
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    loop{
        display.show(&mut timer, lights1, 1000);
    }
}
