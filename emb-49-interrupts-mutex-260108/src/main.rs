#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use microbit::{
    Board, 
    hal::{gpiote::{self, Gpiote}, pac::interrupt}, 
    pac::{self, GPIOTE}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

// GPIOTE -> Connects to Pins and listens for events
// NVIC -> Routes Signal to CPU (When NVIC is masked it ignores interrupt signals)
// ISR -> Function that is executed on trigger

// One Key Aspect of interrupt events are the event flags
// When an interrupt event on a channel occurs it sets the flag to 1
// It will stay at 1 until it is explicitly cleared

// There are different Peripherals that can trigger interrupts
// One of them is GPIOTE, which can watch a total of 8 pins as so called channels
// During configuration the signal fluctuates and it might set the flag to 1 accidentially
// To avoid this we explicitly reset the channel
// Those flags are stored, so it can happen that we receive interrupts that have happened long before
// This means we also need to clear all pending events in the NVIC on start


static SHARED_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn GPIOTE() {
    // This code runs when the interrupt is received
    cortex_m::interrupt::free(|cs| {
        // This passes the key to unlock the Mutex which lets us allow to receive the data 
        let mut gpiote_options = SHARED_GPIOTE.borrow(cs).borrow_mut();
        // We check if successfull and can then reset the channel when received
        // Otherwise it would fire continuously
        match gpiote_options.as_mut() {
            Some(gpiote) =>  {
                let channel0 = gpiote.channel0();
                if channel0.is_event_triggered() {
                    rprintln!("Button A");
                    // We use the rest event method to set the flag for this channel to 0 again
                    // Otherwise the NVIC would see the flag at 1 and force to run this function, over an over again
                    channel0.reset_events();
                }

                let channel1 = gpiote.channel1();
                if channel1.is_event_triggered() {
                    rprintln!("Button B");
                    channel1.reset_events();
                }
            },
            None => (), 
        };
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let button_a = board.buttons.button_a.into_pullup_input();
    let button_b = board.buttons.button_b.into_pullup_input();

    let gpiote = Gpiote::new(board.GPIOTE);
    // This connects the electrical wire from Pin P0.14 (Button A) to GPIOTE Channel 0
    // And tells the hardware to watch this pin and when the voltage goes from hight to low it trigges the EVENT_IN[0] == 1 event
    let channel0 = gpiote.channel0();
    channel0.input_pin(&button_a.degrade()).hi_to_lo().enable_interrupt();
    // The pins might fluctuate on configuration and this would set the Event Triggered Flag to 1, resulting in an event where the is actually none
    // To avoid this we reset the pin and start with a clean slate
    channel0.reset_events();

    let channel1 = gpiote.channel1();
    channel1.input_pin(&button_b.degrade()).hi_to_lo().enable_interrupt();
    channel1.reset_events();

    // Tell the chip that there are no interrupts happening (how dos it do that)
    // It disables the main interrupt switch of the cpu
    // This allows us to entery the Mutex and write to it
    // Then store the gpiote in that variable, so we can use it inside the interrupt
    // This is some setup code that runs once to setup our connection
    cortex_m::interrupt::free(|cs| {
        SHARED_GPIOTE.borrow(cs).replace(Some(gpiote));
        // Is there anything else that we need to do
    });

    // This part deals with the pac, it unmasks and unpends the NVIC
    // Unmask enables the NVIC to receive interrupts
    // Unpend clears all pending interrupt signals 
    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::interrupt::GPIOTE);

    loop {
        asm::wfi();
    }
}