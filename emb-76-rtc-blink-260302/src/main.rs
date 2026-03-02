
#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::board;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut pins = board.display_pins;

    pins.row1.set_high();
    pins.col1.set_low();

    loop {
        asm::wfi();
    }
}
