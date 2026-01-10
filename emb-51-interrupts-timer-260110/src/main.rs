
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

#[entry]
fn main() -> ! {
    loop {}
}
