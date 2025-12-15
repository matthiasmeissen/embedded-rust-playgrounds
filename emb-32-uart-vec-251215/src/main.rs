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

    let mut serial: UartePort<microbit::pac::UARTE0> = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // Create a new heapless vector with capacity of 32 items and of type u8
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        let byte = serial.read().unwrap();
        // Add the input byte to the buffer
        let buffer_result = buffer.push(byte);
        match buffer_result {
            Ok(r) => (),
            Err(e) => rprintln!("Buffer is full"),
        };

        serial.write(byte).unwrap();
        rprintln!("Byte: {}     Char: {}    Buffer: {:?}", byte, byte as char, buffer);

        if byte == 13 {
            serial.write(b'\n').unwrap();
            print_buffer(&mut buffer, &mut serial);
            // Remove all items in the buffer
            buffer.clear();
        }
    }
}

fn print_buffer(buffer: &mut Vec<u8, 32>, serial: &mut UartePort<microbit::pac::UARTE0>) {
    for i in buffer.iter().rev() {
        serial.write(*i).unwrap();
    }
    serial.write(b'\r').unwrap();
    serial.write(b'\n').unwrap();
}