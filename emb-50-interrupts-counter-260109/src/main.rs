#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use microbit::{
    Board, 
    hal::{gpiote::{self, Gpiote}, pac::interrupt}, 
    pac::{self, GPIOTE}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

static SHARED_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

// A Mutex with RefCell works well for complex data types, but requires to lock the interrupts on access
static COUNTER: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

// An Atomic is a more performant way to safely share data that requires no locking and thus less cpu cycles
static COUNTER2: AtomicUsize = AtomicUsize::new(0);

#[interrupt]
fn GPIOTE() {
    // When ISR is called we disable all interrupts, and pass that key to access the gpiote
    cortex_m::interrupt::free(|cs| {
        let mut gpiote_option = SHARED_GPIOTE.borrow(cs).borrow_mut();
        let mut counter = COUNTER.borrow(cs).borrow_mut();
        match gpiote_option.as_mut() {
            Some(gpiote) => {
                // When gpiote is present, execute some code
                // Extract channel0 and check if its event is triggered
                let channel0 = gpiote.channel0();
                if channel0.is_event_triggered() {
                    *counter += 1;
                    channel0.reset_events();
                }

                let channel1 = gpiote.channel1();
                if channel1.is_event_triggered() {
                    COUNTER2.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                    channel1.reset_events();
                }
            },
            None => (),
        }
    })
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let button_a = board.buttons.button_a.into_pullup_input();
    let button_b = board.buttons.button_b.into_pullup_input();

    // Create GPIOTE instance
    let gpiote = Gpiote::new(board.GPIOTE);

    // Create channel0 from GPIOTE and set up to watch button a pin get hight to low that sets EVENT_IN[0] == 1 as Flag
    let channel0 = gpiote.channel0();
    channel0.input_pin(&button_a.degrade()).hi_to_lo().enable_interrupt();
    // Reset all events that might have been created on configuration
    channel0.reset_events();

    let channel1 = gpiote.channel1();
    channel1.input_pin(&button_b.degrade()).hi_to_lo().enable_interrupt();
    channel1.reset_events();

    // Call free() on interrupts, which ensures that all interrupts are disabled currently, pass that as key to Mutex
    cortex_m::interrupt::free(|cs| {
        // Use key to access refcell and replace its value with the gpiote, to enable access in ISR
        SHARED_GPIOTE.borrow(cs).replace(Some(gpiote));
    });

    // Unmask the NVIC to listen to interrupts
    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) };
    // Clear all pending interrupt events that might be present
    pac::NVIC::unpend(pac::interrupt::GPIOTE);

    loop {
        // Set the main loop to low energy by doing nothing but wait for interrupt signals
        asm::wfi();
        let count_value = cortex_m::interrupt::free(|cs| {
            *COUNTER.borrow(cs).borrow()
        });
        let count_value2 = COUNTER2.load(core::sync::atomic::Ordering::Relaxed);
        rprintln!("Count1: {}, Count2: {}", count_value, count_value2);
    };
}