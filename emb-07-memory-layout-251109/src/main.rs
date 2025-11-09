// Lets us use core libarary, sincd std not suitable for embedded devices that dos not run any os on it
#![no_std]
// Without an os there is no main function that is running
#![no_main]

// Instead we use cortex_m_rt to specify entry point for the program
use cortex_m_rt::entry;
// The microbit crate is a BSC with an abstraction layer to access the chip with sensors
use microbit::board::Board;
use microbit::gpio::DisplayPins;
use microbit::hal::Timer;
// The embedded_hal crate provides common methods for embedded devices like .set_high() on pins
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;
// The panic_hal defines how the program should behave on panic, since the is no os it can return to when program terminates for some reason
use panic_halt as _;

// There are two types of memory layout
// Flash:   Non Volatile, Remains the same when power off. Contains the programs binary. Slower.
// Ram:     Volatile, Conten is lost on power off. Stores data program needs to access and modify. Faster.

// Within Flash layout there are three memory sections
// .text    Contains the program itself in machine code
// .data    Contains static variables with predefined value on startup (copies from flash to ram)
// .bss     Special section for static variables that are 0 (also copies from flash to ram)

// To see the size of each of the sections of you program in bytes you can build it and run
// cargo size

// To modify the size of the .data and .bss section you can adjust the value of COUNT
// When you initialize it to 0 it is placing it (using 4 bytes) in the .bss section
// When you initialize it to something else it places it in the .data section
static mut COUNT: i32 = 100;
static WRAP: i32 = 8;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut pins = board.display_pins;
    let mut timer = Timer::new(board.TIMER0);

    let mut flip = false;

    activate_cols(&mut pins, flip);
    pins.row1.set_high().unwrap();

    loop {
        unsafe {
            COUNT += 1;
        }
        timer.delay_ms(200);

        let current_count = unsafe { COUNT };

        if current_count % WRAP / 4 == 0 {
            flip = !flip;
        }

        if current_count % WRAP == 0 {
            pins.row1.set_low().unwrap();
            pins.row2.set_high().unwrap();
        } else {
            pins.row1.set_high().unwrap();
            pins.row2.set_low().unwrap();
        }
        activate_cols(&mut pins, flip);
    }
}

fn activate_cols(pins: &mut DisplayPins, flip: bool) {
    if flip {
        pins.col1.set_low().unwrap();
        pins.col2.set_high().unwrap();
        pins.col3.set_low().unwrap();
        pins.col4.set_high().unwrap();
        pins.col5.set_low().unwrap();
    } else {
        pins.col1.set_high().unwrap();
        pins.col2.set_low().unwrap();
        pins.col3.set_high().unwrap();
        pins.col4.set_low().unwrap();
        pins.col5.set_high().unwrap();
    }
}