#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use microbit::{
    Board, 
    hal::{Timer, gpiote::Gpiote, pac::interrupt}, 
    pac::{self, TIMER0}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

static SHARED_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        let mut gpiote_options = SHARED_GPIOTE.borrow(cs).borrow_mut();
        match gpiote_options.as_mut() {
            Some(gpiote) => {
                let channel0 = gpiote.channel0();
                if channel0.is_event_triggered() {
                    COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                }
                gpiote.reset_events();
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

    let gpiote = Gpiote::new(board.GPIOTE);
    let channel = gpiote.channel0();
    channel.input_pin(&button_a.degrade()).hi_to_lo().enable_interrupt();
    channel.reset_events();

    cortex_m::interrupt::free(|cs| {
        SHARED_GPIOTE.borrow(cs).replace(Some(gpiote));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::interrupt::GPIOTE);

    loop {
        asm::wfi();

        let counter_value = COUNTER.load(core::sync::atomic::Ordering::Relaxed);
        rprintln!("{}", counter_value);
    }
}