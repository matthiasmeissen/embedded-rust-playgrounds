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

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let initial_leds: [[u8; 5]; 5] = [
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
    ];

    let mut leds: [[u8; 5]; 5] = [
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
    ];

    loop {
        for i in 0..5 {
            for j in 0..5 {
                leds[i][j] = 1;
                display.show(&mut timer, leds, (40 * i + 20) as u32);
                leds = initial_leds;
            }
        }
    }
}
