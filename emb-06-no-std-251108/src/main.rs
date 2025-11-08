
// Things to do first
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
use microbit::board::Board;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    // This is not allowed (allocates dynamic memory on the heap) since we are using no_std
    //let nums = vec![1, 2, 3, 4];

    // We need to use a static array with fixed size instead
    let nums = [1, 2, 3, 4];

    loop {}
}
