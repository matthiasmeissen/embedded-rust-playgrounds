#![no_std]
#![no_main]

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use microbit::{
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
    board,
};

mod serial_setup;
use serial_setup::UartePort;


// Get device name          `ls /dev/cu.usbmodem*`
// Launch mincom            `minicom -D /dev/cu.YOURDEVICENAME -b 115200`

// This will open the minicom cli
// To interact with it on a mac you need to use the Meta key (which is ESC Key)
// Press ESC + Z to enter the help menu to see all commands available


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // You can create an array of character bytes to send over serial
    let word1 = [b'H', b'e', b'l', b'l', b'o', b' '];

    for c in word1 {
        serial.write(c).unwrap();
    }

    // We can also use the .bytes() method on a &str to convert it directly
    // Note the \n and \r symbols
    // The \n (line feed) and moves the cursor to next line, but does not return to the beginning
    // The \r (carriage return) moves the cursor to the beginning of a line
    // In combination they move the cursor to beginning and then sets a new line
    for c in "People\r\n".bytes() {
        serial.write(c).unwrap();
    }

    serial.flush().unwrap();

    loop {
        wfi();
    }
}