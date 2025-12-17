#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use heapless::Vec;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use microbit::{ 
    board,
    hal::uarte::{Baudrate, Parity, Uarte},
};

// Get device name          `ls /dev/cu.usbmodem*`
// Launch mincom            `minicom -D /dev/cu.usbmodem2102 -b 115200`

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();

    let mut serial = Uarte::new(
        board.UARTE0, 
        board.uart.into(), 
        Parity::EXCLUDED, 
        Baudrate::BAUD115200
    );

    let buffer = [b'T', b'e', b's', b't', b'\r', b'\n'];
    serial.write(&buffer).unwrap();

    write!(serial, "Another test").unwrap();

    let mut read_buffer: [u8; 1] = [0];

    loop {
        let _ = serial.read(&mut read_buffer).unwrap();

        rprintln!("Byte:    {}, Char:   {}", read_buffer[0], read_buffer[0] as char);

        serial.write(&mut read_buffer).unwrap();
    }
}