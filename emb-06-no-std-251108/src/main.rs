// Set up the program
// Add .cargo/config -> Holds variables and data for compiling the program to embedded target
// Add Embed.toml -> Holds configuration for cargo embed command (which debugger and such)
// Adjust Cargo.toml -> Includes dependecies like PAC, HAL and BAL

// The no_std command is necessary for embedded systems, since they have no os where our program runs in
// Our program is the only thing that us running on that thing
// This means we can not use a lot of things that the std library handles for us (like file system and dynamic memory allocation)
// The hardware we are working with is rather limited, so we need to do memory management with static memory
// We have access the core library with that, which includes a subset of the std libarary only
#![no_std]
// We also do not have a main function, this means we need to specify and antry point for the program
#![no_main]

// This entry is provided by the cortex_m_rt crate and needs to be added as a macro befor the function we want to declare as entry point
// This function can be named whatever we want, in this case it is called main() as well
use cortex_m_rt::entry;
// Since we are the onyl program that is running on that chip we need to handle panic events in a specific way
// The panic_halt crate is one way to do that
use panic_halt as _;
// The embedded_hal crate contains method we can call on peripherals of the board
// For this case it includes the set_low() and set_high() to use on pins in order to light up leds that way
use embedded_hal::{digital::OutputPin, delay::DelayNs};
// The microbit crate is the BSC which includes the connection of the chip with the sensors on a specific board
use microbit::{board::Board, gpio::DisplayPins, hal::Timer};

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut display_pins = board.display_pins;
    let mut timer = Timer::new(board.TIMER0);

    // This is not allowed (allocates dynamic memory on the heap) since we are using no_std
    //let nums = vec![1, 2, 3, 4];

    // We need to use a static array with fixed size instead
    let nums = [2, 4, 1, 5, 3];
    let mut index = 0;

    
    loop {
        light(&mut display_pins, nums[index]);
        
        timer.delay_ms(400);

        index += 1;
        if index >= nums.len() {
            index = 0;
        }
    }
}

// This function takes a mutable reference to board so it can modify it without taking ownerhip
// fn own(mut board: Board) {}
// In contrast this would take owndership so one we call it we wont be able to call it another time
fn led1(board: &mut Board) {
    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();
}

fn led2(board: &mut Board) {
    board.display_pins.col2.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();
}

fn light(pins: &mut DisplayPins, num: usize) {
    pins.col1.set_low().unwrap();

    pins.row1.set_low().unwrap();
    pins.row2.set_low().unwrap();
    pins.row3.set_low().unwrap();
    pins.row4.set_low().unwrap();
    pins.row5.set_low().unwrap();

    match num {
        1 => pins.row1.set_high().unwrap(),
        2 => pins.row2.set_high().unwrap(),
        3 => pins.row3.set_high().unwrap(),
        4 => pins.row4.set_high().unwrap(),
        5 => pins.row5.set_high().unwrap(),
        _ => ()
    }
}