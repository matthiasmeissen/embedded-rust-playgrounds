
#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::{AtomicU32, AtomicUsize, Ordering}, u32};

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

static MAIN_PHASOR: AtomicU32 = AtomicU32::new(0);
static PHASE_INCREMENT: AtomicU32 = AtomicU32::new(0);

static LFO_1: AtomicU32 = AtomicU32::new(0);
static LFO_2: AtomicU32 = AtomicU32::new(0);

const TIMER_PERIOD_US: u32 = 1_000;

#[interrupt]
fn TIMER0() {
    static mut HALF_PHASOR: u32 = 0;
    
    critical_section::with(|cs| {
        let mut timer_options = SHARED_TIMER.borrow(cs).borrow_mut();
        match timer_options.as_mut() {
            Some(timer) => {
                if timer.events_compare[0].read().bits() != 0 {
                    timer.events_compare[0].write(|w| unsafe { w.bits(0)});

                    // Increment main phasor
                    let inc = PHASE_INCREMENT.load(Ordering::Relaxed);
                    let main = MAIN_PHASOR.load(Ordering::Relaxed);
                    let (new_main, _) = main.overflowing_add(inc);
                    
                    // Double Speed
                    let lfo1 = new_main.wrapping_mul(2);
                    
                    // Half Speed
                    *HALF_PHASOR = HALF_PHASOR.wrapping_add(inc >> 1);
                    let lfo2 = *HALF_PHASOR;
                    
                    MAIN_PHASOR.store(new_main, Ordering::Relaxed);
                    LFO_1.store(lfo1, Ordering::Relaxed);
                    LFO_2.store(lfo2, Ordering::Relaxed);
                }
            },
            None => ()
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    // Create a new Timer from Board, enable interrupts
    let mut timer = Timer::new(board.TIMER0);
    timer.enable_interrupt();
    // Convert Timer into periodic
    let mut timer = timer.into_periodic();
    // Set timer to 100_000 us interval
    timer.start(TIMER_PERIOD_US);
    // Get access to raw interface of peripheral
    let timer = timer.free();

    critical_section::with(|cs| {
        SHARED_TIMER.borrow(cs).replace(Some(timer));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::TIMER0) };
    pac::NVIC::unpend(pac::interrupt::TIMER0);

    PHASE_INCREMENT.store(get_increment_from_bpm(120.0, TIMER_PERIOD_US), Ordering::Relaxed);

    rprintln!("Setup done");

    loop {
        asm::wfi();

        let main_phasor = normalize_phasor(MAIN_PHASOR.load(Ordering::Relaxed));
        let lfo_1 = normalize_phasor(LFO_1.load(Ordering::Relaxed));
        let lfo_2 = normalize_phasor(LFO_2.load(Ordering::Relaxed));
        rprintln!("Main: {:.2}      Double: {:.2}      Half: {:.2}", main_phasor, lfo_1, lfo_2);
    }
}

fn get_increment_from_bpm(bpm: f32, interval_us: u32) -> u32 {
    let beats_per_second = bpm / 60.0;
    let seconds_per_bar = 4.0 / beats_per_second;
    let ticks_per_bar = seconds_per_bar / (interval_us as f32 / 1_000_000.0);
    (u32::MAX as f32 / ticks_per_bar) as u32
}

fn normalize_phasor(value: u32) -> f32 {
    (value as f32) / (u32::MAX as f32)
}
