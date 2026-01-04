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


// Interrupts are used to wake the cpu up to perform a task
// This is better than polling, where the cpu is always busy and cannot do anything else while doing that, it also wastes energy
// When the cpu receives and interrupt it stores current state in stack, performs ISR, loads back state

// ISR (Interrupt Service Routine) - The function that is called when interrupt is received, marked with #[interrupt]
// GPIOTE (GPIO Task and Events) - Pins on the chip that listen for GPIO events, has 8 channels that can listen to one pin each
// NVIC (Nested Vector Interrupt Controller) - Stores pointers to ISR to call them when interrupt is received


#[interrupt]
fn GPIOTE() {
    rprintln!("Done");
    panic!();
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let button_b = board.buttons.button_b.into_pullup_input();

    let gpiote = gpiote::Gpiote::new(board.GPIOTE);
    let channel0 = gpiote.channel0();
    channel0.input_pin(&button_b.degrade()).hi_to_lo().enable_interrupt();
    channel0.reset_events();

    // Interrupts are masked by default, unmask to enable
    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) }
    // Clear any pending interrupt events
    pac::NVIC::unpend(pac::interrupt::GPIOTE);

    loop {
        asm::wfi();
    }
}