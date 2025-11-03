#![deny(unsafe_code)]
#![no_main]
#![no_std]

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
    // This creates a new timer instance somehow related to Timer0
    // This seems to be one of many different timers in this microcontroller
    let mut timer = Timer::new(board.TIMER0);
    // To light up an led you need to set its col pin to low and its row pin to high
    board.display_pins.row1.set_high().unwrap();

    loop {
        board.display_pins.col1.set_low().unwrap();
        board.display_pins.col2.set_high().unwrap();
        timer.delay_ms(500);

        board.display_pins.col1.set_high().unwrap();
        board.display_pins.col2.set_low().unwrap();
        timer.delay_ms(500);
    }
}
