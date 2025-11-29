#![no_main]
#![no_std]


// Basic Commands
// Build:           cargo build
// Inspect:         cargo readobj --target thumbv7em-none-eabihf --bin registers -- --file-header
// Flash:           cargo embed

// New Terminal
// Open GDB:        arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/registers
// Connect:         target remote :1337
// Set Mem:         set mem inaccessible-by-default off
// Add Breakpoint:  break main.rs:27
// Continue:        continue


#[allow(unused_imports)]
use registers::entry;

#[entry]
fn main() -> ! {
    let (p0, _p1) = registers::init();

    // Turn on the top row
    p0.out.modify(|_, w| w.pin21().set_bit());

    // Turn on the bottom row
    p0.out.modify(|_, w| w.pin19().set_bit());

    // Turn off the top row
    p0.out.modify(|_, w| w.pin21().clear_bit());

    // Turn off the bottom row
    p0.out.modify(|_, w| w.pin19().clear_bit());

    loop {}
}