
#![no_std]
#![no_main]

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
// Launch mincom            `minicom -D /dev/cu.YOURDEVICENAME -b 115200`


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

    print_line(INTRO, &mut serial);
    serial.flush().unwrap();

    loop {
        wfi();
    }
}
