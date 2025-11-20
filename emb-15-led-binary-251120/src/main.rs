#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board, display::blocking::Display, hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut bit_display: [u8; 5] = [
        0b11111,
        0b10001,
        0b10001,
        0b10001,
        0b11111,
    ];

    let converted_display = binary_to_array(&mut bit_display);

    loop {
        display.show(&mut timer, converted_display, 1000);
        timer.delay_ms(200);
    }
}

fn binary_to_array(input: &mut [u8; 5]) -> [[u8; 5]; 5] {
    let mut output = [[0; 5]; 5];

    for i in 0..5 {
        output[i] = bit_to_row(input[i]);
    }

    output
}

fn bit_to_row_complex(input: u8) -> [u8; 5] {
    match input {
        0b11111 => [1, 1, 1, 1, 1],
        0b10001 => [1, 0, 0, 0, 1],
        _ => [0, 0, 0, 0, 0],
    }
}

fn bit_to_row(input: u8) -> [u8; 5] {
    let mut row= [0; 5];

    for i in 0..5 {
        // Usually when we have a binary number like 0b001, the 1 is at position 0
        // We want to read bits from left to right, so we need to flip it
        let bit_position = 4 - i;

        // The >> shifts the binary number by a specific amount to the right
        // Bits that fall off the right side are lost and bits comming from the left are filled with 0
        // Example: 0b010 >> 1 -> 0b001
        // Example: 0b100 >> 2 -> 0b001
        let shifted = input >> bit_position;

        // The bitwise AND operator & is used to isolate it and get the value
        // Example 1 & 1 is 1, 0 & 1 is 0, 0 & 0 is 0
        let bit_on = shifted & 1;

        row[i] = bit_on;
    }

    row
}