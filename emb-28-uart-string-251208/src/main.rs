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

// This will open the minicom cli
// To interact with it on a mac you need to use the Meta key (which is ESC Key)
// Press ESC + Z to enter the help menu to see all commands available

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    let mut serial: UartePort<microbit::pac::UARTE0> = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    const INTRO: &str = "Program started\r\n";
    const PATTERN_A: &str = "#_#_#_#_#_#\r\n";
    const PATTERN_B: &str = "#_________#\r\n";

    print_line(INTRO, &mut serial);

    loop {
        if button_a.is_low().unwrap() {
            print_line(PATTERN_A, &mut serial);
            timer.delay_ms(200);
        }

        if button_b.is_low().unwrap() {
            print_line(PATTERN_B, &mut serial);
            timer.delay_ms(200);
        }

        serial.flush().unwrap();

        timer.delay_ms(20);
    }
}

fn print_line(input: &str, serial: &mut UartePort<microbit::pac::UARTE0>) {
    let byte_string = input.bytes();
    for i in byte_string {
        serial.write(i).unwrap();
    }
}