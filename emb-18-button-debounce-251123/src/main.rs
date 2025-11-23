
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{Board, board::{self, Buttons}, display::blocking::Display, hal::Timer};
use panic_halt as _;

const DISPLAY1: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];

const DISPLAY2: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
];

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    //let mut display = Display::new(board.display_pins);

    let mut pins = board.display_pins;

    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();


    let button_a = board.buttons.button_a.into_pullup_input();
    let button_b = board.buttons.button_b.into_pullup_input();

    let a_pressed = false;
    let b_pressed = false;

    loop {
        timer.delay_ms(200);
    }
}
