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
use microbit::{board::Board, hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.col1.set_low();
    board.display_pins.row1.set_high();

    loop {}
}
