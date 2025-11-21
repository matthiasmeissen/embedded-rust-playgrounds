#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::{InputPin, OutputPin}, delay::DelayNs};
use microbit::{board, display::blocking::Display, gpio::{BTN_A, DisplayPins}, hal::Timer, pac::TIMER0};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let pattern_1: [u8; 5] = [
        0b00100,
        0b01000,
        0b11111,
        0b01000,
        0b00100,
    ];

    let pattern_2: [u8; 5] = [
        0b00100,
        0b00010,
        0b11111,
        0b00010,
        0b00100,
    ];

    loop{
        // Input reading at button is low (0) when pressed
        if button_a.is_low().unwrap() {
            show_pattern(&mut display, &mut timer, pattern_1);
        } else {
            display.clear();
        }

        if button_b.is_low().unwrap() {
            show_pattern(&mut display, &mut timer, pattern_2);
        } else {
            display.clear();
        }
    }
}

fn show_pattern(display: &mut Display, timer: &mut Timer<TIMER0>, pattern_input: [u8; 5]) {
    let pattern = row_array_to_pattern(pattern_input);
    display.show(timer, pattern, 20);
}

fn row_array_to_pattern(input: [u8; 5]) -> [[u8; 5]; 5] {
    let mut array = [[0; 5]; 5];
    for i in 0..5 {
        array[i] = binary_to_row_array(input[i]);
    };
    array
}

fn binary_to_row_array(input: u8) -> [u8; 5] {
    let mut row = [0; 5];
    for i in 0..5 {
        let pos = 4 - i;
        let shift = input >> pos;
        let num = shift & 1;
        row[i] = num;
    }
    row
}