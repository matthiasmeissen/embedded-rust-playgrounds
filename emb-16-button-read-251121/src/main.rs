
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::{InputPin, OutputPin}, delay::DelayNs};
use microbit::{board, gpio::{DisplayPins, BTN_A}};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut pins = board.display_pins;

    let mut button_a = board.buttons.button_a;

    pins.row3.set_high().unwrap();
    pins.col3.set_low().unwrap();

    loop{
        if button_a.is_low().unwrap() {
            pins.col3.set_high().unwrap();
        } else {
            pins.col3.set_low().unwrap();
        }
    }
}
