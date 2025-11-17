
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let shape = [
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
    ];

    loop {
        display.show(&mut timer, shape, 1000);
        display.clear();
        timer.delay_ms(200);
    }
}
