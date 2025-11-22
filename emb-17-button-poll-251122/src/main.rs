#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{board, display::blocking::Display, hal::Timer};
use panic_halt as _;

const LIGHTS1: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
];

const LIGHTS2: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
];

const LIGHTS3: [[u8; 5]; 5]    = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
];

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    loop{
        let pressed_a = button_a.is_low().unwrap();
        let pressed_b = button_b.is_low().unwrap();

        match (pressed_a, pressed_b) {
            (true, false) => display.show(&mut timer, LIGHTS1, 20),
            (false, true) => display.show(&mut timer, LIGHTS2, 20),
            (true, true) => display.show(&mut timer, LIGHTS3, 20),
            (false, false) => {display.clear();timer.delay_ms(20);}
        }
    }
}