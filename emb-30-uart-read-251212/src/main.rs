
#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use embedded_hal::{delay::DelayNs, digital::InputPin};

use microbit::{
    board, hal::{Timer, uarte::{self, Baudrate, Parity}}
};

mod serial_setup;
use serial_setup::UartePort;


// Get device name          `ls /dev/cu.usbmodem*`
// Launch mincom            `minicom -D /dev/cu.usbmodem2102 -b 115200`

#[entry]
fn main() -> ! {
    loop {}
}
