#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::{asm};
use cortex_m_rt::entry;
use critical_section::Mutex;
use microbit::{
    Board, 
    hal::{Timer, pac::interrupt}, 
    pac::{self, TIMER0}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

static SHARED_TIMER: Mutex<RefCell<Option<TIMER0>>> = Mutex::new(RefCell::new(None));
static SHARED_COUNTER: AtomicUsize = AtomicUsize::new(0);


#[interrupt]
fn TIMER0() {
    critical_section::with(|cs| {
        let mut timer_options = SHARED_TIMER.borrow(cs).borrow_mut();
        match timer_options.as_mut() {
            Some(timer) => {
                // Check if the event triggered (register is not 0)
                if timer.events_compare[0].read().bits() != 0 {
                    // Clear the event by writing 0 to the register
                    timer.events_compare[0].write(|w| unsafe {w.bits(0)});
                    // Increment the timer
                    SHARED_COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
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

    let mut timer = Timer::new(board.TIMER0);
    timer.enable_interrupt();
    let mut timer = timer.into_periodic();
    timer.start(100_000u32);

    let raw_timer = timer.free();
    
    critical_section::with(|cs| {
        SHARED_TIMER.borrow(cs).replace(Some(raw_timer));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::TIMER0) };
    pac::NVIC::unpend(pac::interrupt::TIMER0);

    loop {
        asm::wfi();

        let counter_value = SHARED_COUNTER.load(core::sync::atomic::Ordering::Relaxed);
        rprintln!("{}", counter_value);
    }
}