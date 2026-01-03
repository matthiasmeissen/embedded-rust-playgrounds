#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit::{
    Board, 
    hal::{gpiote, pac::interrupt}, 
    pac
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};


// Why interrupts
// Polling (if SomeEvent { Action }) keeps cpu always busy 
// this makes hard to do things simultaneously and draws unneccesary power
// Interrupts wake up the cpu to do a specifc task
// Use wfi() function in main loop to set cpu in low power mode

// When an interrupt is detected the cpu 
// - Stores its current state on stack memory
// - Exectutes the code from interrupt function
// - Loads state back and continues with previous task

// Important terms and concepts
// ISR (Interrupt Service Routine) - The function that is called when Interrupt fires, marked with #[interrupt]
// GPIOTE (GPIO Tasks and Events) - Monitors GPIO pins and generates interrupt signals, the microbit has 8 channels, each can monitor one pin
// NVIC (Nested Vectored Interrupt Controller) - Routes interrupt signals to ISR Functions
// Interrupt Vector Table - Table in memory containing pointers to all ISR functions


// ISR (Interrupt Service Routine) - Function that is called when interrupt fires
#[interrupt]
fn GPIOTE() {
    rprintln!("Interrupt");
    panic!();
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let button_a = board.buttons.button_a.into_pullup_input();

    // GPIOTE (GPIO Tasks and Events) - Monitors GPIO pins and generates interrupt signals
    let gpiote = gpiote::Gpiote::new(board.GPIOTE);
    // The microbit has 8 channels, where each can monitor one pin
    let channel = gpiote.channel0();
    channel
        .input_pin(&button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    // Clear any events set during setup
    channel.reset_events();

    // NVIC (Nested Vectored Interrupt Controller) - Routes interrupt Signals to ISR functions
    // Interrupts are masked by default, unmask to enable
    // Is unsafe, since interrupts can fire at any time, creating potential data races
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    // Clear any pending interrupts from setup
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);

    loop {
        asm::wfi();
    }
}