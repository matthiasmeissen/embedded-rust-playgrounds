#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use microbit::{
    Board, 
    hal::{self, Timer, gpiote::Gpiote, pac::interrupt}, 
    pac::{self, TIMER0}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

static SHARED_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static SHARED_TIMER: Mutex<RefCell<Option<Timer<TIMER0>>>> = Mutex::new(RefCell::new(None));

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        cortex_m::interrupt::free(|cs| {
        if let Some(timer) = SHARED_TIMER.borrow(cs).borrow_mut().as_mut() {
            let reading = timer.read();
            rprintln!("ISR entry, timer = {}", reading);
            
            if reading == 0 {
                let new_count = COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed) + 1;
                timer.start(200_000u32);
                rprintln!("Press accepted! Count = {}", new_count);
                
                // Verify timer actually started
                let after_start = timer.read();
                rprintln!("Timer after start() = {}", after_start);
            } else {
                rprintln!("Debounce active ({}), ignoring", reading);
            }
        } else {
            rprintln!("ERROR: Timer is None!");
        }
        
        if let Some(gpiote) = SHARED_GPIOTE.borrow(cs).borrow_mut().as_mut() {
            gpiote.channel0().reset_events();
        } else {
            rprintln!("ERROR: GPIOTE is None!");
        }
    });
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let button_a = board.buttons.button_a.into_pullup_input();

    let gpiote = Gpiote::new(board.GPIOTE);
    let channel0 = gpiote.channel0();
    channel0.input_pin(&button_a.degrade()).hi_to_lo().enable_interrupt();
    channel0.reset_events();

    let mut timer0 = Timer::new(board.TIMER0);
    timer0.disable_interrupt();
    timer0.reset_event();

    timer0.start(100_000);
    rprintln!("Timer Started");

    // for i in 0..200 {
    //     let reading = timer0.read();
    //     rprintln!("  [{}] read() = {}", i, reading);
        
    //     for _ in 0..1_000 {
    //         cortex_m::asm::nop();
    //     }

    //     if reading == 0 {
    //         rprintln!("Restart Timer");
    //         timer0.start(100_000);
    //     }
    // }

    cortex_m::interrupt::free(|cs| {
        SHARED_GPIOTE.borrow(cs).replace(Some(gpiote));
        SHARED_TIMER.borrow(cs).replace(Some(timer0));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::interrupt::GPIOTE);

    loop {
        asm::wfi();
        let counter_value = COUNTER.load(core::sync::atomic::Ordering::Relaxed);
        rprintln!("{}", counter_value);
    }
}