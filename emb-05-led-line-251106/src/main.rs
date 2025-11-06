#![deny(unsafe_code)]
#![no_std]
#![no_main]

// Basic Commands
// Build:       cargo build
// Inspect:     cargo readobj --target thumbv7em-none-eabihf --bin led-blink -- --file-header
// Flash:       cargo embed
// New Terminal
// Open GDB:    arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/led-blink
// Connect:     target remote :1337

// This create defines the no_main entry point for our program, this might be the direct periphery access layer
use cortex_m_rt::entry;
// The embedded hal is the abstraction layer that provides the traits used to communicate with the chip, but not used here
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;
// The microbit library includes a nother abstraction layer above hal that si tied directly to the board with the chip and its connected sensors
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let initial_leds: [[u8; 5]; 5] = [[0; 5]; 5];
    let mut leds: [[u8; 5]; 5] = [[0; 5]; 5];

    let mut num = 0;

    loop {

        for i in 0..5 {
            for j in 0..5 {
                leds[i][j] = 1;
                leds[num % 5][i] = 1;
                leds[(num + i) % 4][num % 2] = 1;
            }
            display.show(&mut timer, leds, 200);
            leds = initial_leds;
        }
        num += 1;
    }
}
