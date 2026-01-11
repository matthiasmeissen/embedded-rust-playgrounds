
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

#[entry]
fn main() -> ! {
    loop {}
}
