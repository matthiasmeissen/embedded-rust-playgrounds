#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use microbit::{
    Board, 
    hal::{gpiote::{self, Gpiote}, pac::interrupt}, 
    pac
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

// It is not possible to use a global variable
// that is shared between the main program and the interrupt
// since it might create race conditions
// static mut COUNTER: i32 = 0;

// This pattern solves it
// <Option<T>> 
// We need to define a type for the variable, on start main did not run yet so it is of type None, when it is done it is Gpiote
// An Option is perfect for this, since it is either None or Some(T)

// <RefCell<T>>
// When you declare some thing as static it is immutable (read only)
// The RefCell lets you change something inside that static variable (this is called Interior Mutability)

// <Mutex<T>>
// The RefCell is not thread safe, so we could still have race conditions (interrupt and main try to access at same time)
// A Mutex on a PC says "wait here until the other thread is done"
// A Mutex on an embedded system is different. It says "I will only open, when you prove me that you have disabled all other interrupts"
// This is done by the cs token, which by defintion proves that the main loop can not be running
static GLOBAL_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));


#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        let ref_cell = GLOBAL_GPIOTE.borrow(cs);
        let mut shared_data = ref_cell.borrow_mut();

        match shared_data.as_mut() {
            Some(gpiote) => {
                let channel0 = gpiote.channel0();

                if channel0.is_event_triggered() {
                    // The .reset_events() method clears the interrupt notification, so it does not fire continuosly
                    channel0.reset_events();
                    rprintln!("Test");
                }
            },
            None => ()
        }
    })
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let button_a = board.buttons.button_a.into_pullup_input();

    let gpiote = gpiote::Gpiote::new(board.GPIOTE);
    let channel = gpiote.channel0();
    channel.input_pin(&button_a.degrade()).hi_to_lo().enable_interrupt();
    channel.reset_events();
    
    // When the free() method is called it guarantees that there are no other interrupts happening
    // It creates a key and passes it into the closure
    // This is called critical section
    cortex_m::interrupt::free(|cs| {
        // Get RefCell from Mutex by showing it the critical section key
        let ref_cell = GLOBAL_GPIOTE.borrow(cs);
        // We use .borrow_mut() on the ref cell to get a mutable reference
        let mut content = ref_cell.borrow_mut();
        *content = Some(gpiote);
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::interrupt::GPIOTE);

    loop {
        asm::wfi();
    };
}