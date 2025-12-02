#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{InputPin,OutputPin}};
use microbit::{board, hal::Timer};
use panic_halt as _;


// GDB
// High speed bidirectional communication channel between embedded device and host computer

// Preparation
// To use gdb you need to have the toolchain installed
// And in Embed.toml set 
//          [default.reset] halt_afterwards = true (this means to stop after flashing the program)
//          [default.gdb] enabled = true

// Usage
// Terminal 1
// Start the server:    `cargo embed`

// Terminal 2
// Run GDB              `arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/debug-test`
// Connect Server       `target remote :1337`
// Stop the chip        `monitor reset halt`
// Set breakpoint       `break main.rs:33`
// Continue             `continue`
// Turn on Visual Mode  `layout src`


#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut pins = board.display_pins;

    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();

    let mut count = 0;

    loop {
        pins.col1.set_high().unwrap();
        timer.delay_ms(200);

        pins.col1.set_low().unwrap();
        timer.delay_ms(200);

        count += 1;
    }
}