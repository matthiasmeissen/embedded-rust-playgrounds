
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

#[entry]
fn main() -> ! {
    loop {}
}
