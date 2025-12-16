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


const SHAPE: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];

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

    // Create new heapless vec of type u8 with capacity of 32
    let mut buffer: Vec<u8, 32> = Vec::new();

    print_display(&mut serial);

    loop {
        // Read incoming byte from serial (blocking)
        let byte = serial.read().unwrap();

        // Print the raw byte and the char equivalent to the rtt console
        rprintln!("Byte: {},        Char: {},       Buffer: {:?}", byte, byte as char, buffer);

        // Check if byte is Enter or \r (reset to beginning)
        if byte == b'\r' {
            print_buffer_reverse(&mut serial, &mut buffer);
            // Clear all items from the buffer
            buffer.clear();
        } else {
            // Add byte to buffer
            let update_buffer = buffer.push(byte);
            match update_buffer {
                Ok(a) => (),
                Err(e) => {
                    rprintln!("The buffer is full, will print and clear.");
                    print_buffer_reverse(&mut serial, &mut buffer);
                    buffer.clear();
                },
            }
            // Write byte to serial (blocking)
            serial.write(byte).unwrap();
        }
    }
}

fn print_buffer_reverse(serial: &mut UartePort<microbit::pac::UARTE0>, buffer: &mut Vec<u8, 32>) {
    serial.write(b'\n').unwrap();
    serial.write(b'\r').unwrap();
    for i in buffer.iter().rev() {
        serial.write(*i).unwrap();
    }
    serial.write(b'\n').unwrap();
    serial.write(b'\r').unwrap();
}

fn print_display(serial: &mut UartePort<microbit::pac::UARTE0>) {
    for i in SHAPE {
        for j in i {
            if j == 1 {
                serial.write(b'X').unwrap();
            } else {
                serial.write(b' ').unwrap();
            }
        }
        serial.write(b'\r').unwrap();
        serial.write(b'\n').unwrap();
    }
}