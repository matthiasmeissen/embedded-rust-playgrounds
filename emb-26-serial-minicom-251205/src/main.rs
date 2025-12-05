
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board, hal::Timer};
use panic_halt as _;


#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut  pins = board.display_pins;

    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();

    loop {}
}
