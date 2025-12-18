#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::fmt::Write;
use heapless::Vec;
use microbit::{board, hal::uarte::{Uarte, Parity, Baudrate}};
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

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

    let mut tx_buffer: [u8; 1] = [b'0'];
    let mut rx_buffer: [u8; 1] = [b'0'];

    let mut main_buffer: Vec<u8, 32> = Vec::new();

    rprintln!("Transmitter Buffer is: {:?}", tx_buffer);

    loop {
        serial.read(&mut rx_buffer).unwrap();
        rprintln!("Byte: {},    Char: {}", rx_buffer[0], rx_buffer[0] as char);

        if rx_buffer[0] == b'\r' {
            rprintln!("Print buffer in reverse");

            write!(serial, "\n\r").unwrap();

            for i in main_buffer.iter().rev().chain(&[b'\n', b'\r']) {
                tx_buffer[0] = *i;
                serial.write(&tx_buffer).unwrap();
            }

            rprintln!("Clear buffer");
            main_buffer.clear();
        } else {
            let result = main_buffer.push(rx_buffer[0]);
            match result {
                Ok(_) => {
                    tx_buffer[0] = rx_buffer[0];
                    serial.write(&tx_buffer).unwrap();
                },
                Err(_) => rprintln!("Buffer is full"),
            };
        }
    }
}