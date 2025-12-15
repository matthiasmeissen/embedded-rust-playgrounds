#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use heapless::Vec;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use embedded_hal::{delay::DelayNs, digital::InputPin};

use microbit::{ board, display::blocking::Display, hal::{Timer, uarte::{self, Baudrate, Parity}}};

mod serial_setup;
use serial_setup::UartePort;


// Get device name          `ls /dev/cu.usbmodem*`
// Launch mincom            `minicom -D /dev/cu.usbmodem2102 -b 115200`


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut serial: UartePort<microbit::pac::UARTE0> = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    loop {
        let byte = serial.read().unwrap();
        serial.write(byte).unwrap();

        rprintln!("Byte: {}     Char: {}", byte, byte as char);
    }
}
