
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{board, display::blocking::Display, hal::Timer};
use panic_halt as _;


#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a.into_pullup_input();

    let mut lights = [[1; 5]; 5];

    loop {
        display.show(&mut timer, lights, 1000);
    }
}
