
#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{
    Board, hal::{Timer, gpiote::Gpiote, pac::interrupt}, pac::{self, TIMER0, rtc0::COUNTER}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

fn main() {
    println!("Hello, world!");
}
