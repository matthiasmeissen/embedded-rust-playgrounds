
#![no_std]
#![no_main]

use core::i32;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs};
use microbit::{
    board, 
    hal::{Timer, twim}, 
    pac::twim0::{frequency::FREQUENCY_A},
};
use lsm303agr::{MagMode, MagOutputDataRate, Lsm303agr};
use rtt_target::{rtt_init_print, rprintln};
use libm::{atan2f, sqrtf};
use panic_rtt_target as _;

fn main() {
    println!("Hello, world!");
}
