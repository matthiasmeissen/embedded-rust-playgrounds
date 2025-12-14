#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
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

    let mut buffer: [u8; _] = [0; 20];
    let mut current_index = 0;

    loop {
        let byte = serial.read().unwrap();
        
        // This makes typing absolutely weird, but creates interesting patterns in the console
        match byte {
            119 => {move_writehead(&mut serial, Direction::Up); store_in_buffer(119, &mut buffer, &mut current_index);},            //w
            100 => {move_writehead(&mut serial, Direction::Right); store_in_buffer(100, &mut buffer, &mut current_index);}          //d
            115 => {move_writehead(&mut serial, Direction::Down); store_in_buffer(115, &mut buffer, &mut current_index);}           //s
            97 => {move_writehead(&mut serial, Direction::Left); store_in_buffer(97, &mut buffer, &mut current_index);}             //a

            117 => {move_writehead(&mut serial, Direction::Up); store_in_buffer(117, &mut buffer, &mut current_index);}             //u
            107 => {move_writehead(&mut serial, Direction::Right); store_in_buffer(107, &mut buffer, &mut current_index);}          //k
            106 => {move_writehead(&mut serial, Direction::Down); store_in_buffer(106, &mut buffer, &mut current_index);}           //j
            104 => {move_writehead(&mut serial, Direction::Left); store_in_buffer(104, &mut buffer, &mut current_index);}           //h

            99 => {
                for i in buffer {
                    serial.write(i).unwrap();
                }
            }

            _ => serial.write(byte).unwrap(),
        };

        rprintln!("Byte: {}     Char: {}", byte, byte as char);
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn move_writehead(serial: &mut UartePort<microbit::pac::UARTE0>, direction: Direction) {
    serial.write(27).unwrap();
    serial.write(91).unwrap();

    match direction {
        Direction::Up => serial.write(65).unwrap(),
        Direction::Right => serial.write(67).unwrap(),
        Direction::Down => serial.write(66).unwrap(),
        Direction::Left => serial.write(68).unwrap(),
    }
}

fn store_in_buffer(byte: u8, buffer: &mut [u8; 20], current_index: &mut usize) {
    if *current_index >= buffer.len() {
        *current_index = 0;
    }
    buffer[*current_index] = byte;
    *current_index += 1;
}