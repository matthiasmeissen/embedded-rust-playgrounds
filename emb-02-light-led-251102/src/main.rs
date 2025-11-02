#![deny(unsafe_code)]
#![no_main]
#![no_std]

// No std means there is no standard libarary
// This means no Vec, no String, no println!() and so on

// No main means we do not emit a main symbol from the program
// To define the entry point of the program we import the #[entry] macro from cortex_m_rt crate
// Note that the function is still called main() but could be anything
// Also note that we return ! from it, which means that our program never terminates, it will always keep running

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit as _;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {
        asm::nop();
    }
}
