#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::{
    board,
    hal::gpio::{Level, Output, Pin, PushPull},
    pac::{self, interrupt, RTC0},
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

static RTC: Mutex<RefCell<Option<RTC0>>> = Mutex::new(RefCell::new(None));
static LED_PIN: Mutex<RefCell<Option<Pin<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static LED_STATE: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

// How many ticks per compare interval.
// With PRESCALER=0, frequency is 32768 Hz.
// 32768 ticks = 1 second. 16384 = 0.5s, etc.
const INTERVAL_TICKS: u32 = 32768; // 1 second

#[interrupt]
fn RTC0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(rtc) = RTC.borrow(cs).borrow().as_ref() {
            // Check if COMPARE0 event fired
            if rtc.events_compare[0].read().bits() != 0 {
                // Clear the event
                rtc.events_compare[0].write(|w| unsafe { w.bits(0) });

                // Schedule next compare: current CC + interval (mask to 24 bits)
                let current = rtc.cc[0].read().bits();
                let next = (current + INTERVAL_TICKS) & 0x00FF_FFFF;
                rtc.cc[0].write(|w| unsafe { w.compare().bits(next) });

                // Toggle the LED
                if let Some(pin) = LED_PIN.borrow(cs).borrow_mut().as_mut() {
                    let mut state = LED_STATE.borrow(cs).borrow_mut();
                    *state = !*state;
                    if *state {
                        let _ = pin.set_high();
                    } else {
                        let _ = pin.set_low();
                    }
                }
            }
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = board::Board::take().unwrap();

    let row1 = board.display_pins.row1.into_push_pull_output(Level::High);
    let col1 = board.display_pins.col1.into_push_pull_output(Level::High);

    board.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
    while board.CLOCK.events_lfclkstarted.read().bits() == 0 {}

    let rtc = board.RTC0;
    rtc.prescaler.write(|w| unsafe { w.prescaler().bits(0) }); // 32768 Hz
    rtc.cc[0].write(|w| unsafe { w.compare().bits(INTERVAL_TICKS) });  // First compare
    rtc.evtenset.write(|w| w.compare0().set());                  // Enable compare0 event
    rtc.intenset.write(|w| w.compare0().set());                  // Enable compare0 interrupt
    
    cortex_m::interrupt::free(|cs| {
        LED_PIN.borrow(cs).replace(Some(col1.degrade()));
        RTC.borrow(cs).replace(Some(rtc));
        if let Some(rtc) = RTC.borrow(cs).borrow().as_ref() {
            rtc.tasks_start.write(|w| unsafe { w.bits(1) });
        }
    });
    
    unsafe { pac::NVIC::unmask(pac::interrupt::RTC0) };

    rprintln!("RTC running, toggling LED every 1s");

    loop {
        asm::wfi();
    }
}
