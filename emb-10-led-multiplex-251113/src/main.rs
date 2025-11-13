
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{Board, Peripherals, hal::Timer};
use panic_halt as _;


#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut pins = board.display_pins;
    let mut timer0 = Timer::new(board.TIMER0);

    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();

    loop {
        timer0.delay_ms(200);
        pins.col1.set_high().unwrap();
        timer0.delay_ms(200);
        pins.col1.set_low().unwrap();
    }
}
