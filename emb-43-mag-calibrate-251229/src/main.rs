
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board, 
    hal::{Timer, twim}, 
    pac::twim0::{frequency::FREQUENCY_A},
};
use lsm303agr::{MagMode, MagOutputDataRate, Lsm303agr};
use rtt_target::{rtt_init_print, rprintln};
use libm::sqrtf;
use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    loop {}
}
