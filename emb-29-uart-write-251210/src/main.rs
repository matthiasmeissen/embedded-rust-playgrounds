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


// Basic MIDI
// This is the midi clock status byte
// It is sent from the host to the receiver in regular intervals
// Where 24 of those pulses mark a quarter note
const MIDI_CLOCK_STATUS: u8 = 0xF8;

/*
             1,000,000 * 60
Delay (us) = --------------
                BPM * 24
*/

// For 120 bpm this would be 20,833 us

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut serial: UartePort<microbit::pac::UARTE0> = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    timer.start(0xFFFFFFFF_u32);

    let target_interval_us: u32 = get_clock_pulse_for_bpm(120);
    let mut next_target_time = 0;

    let mut count = 0;

    write!(serial, "Starting midi clock with bpm:: {}\r\n", 120).unwrap();

    loop {
        
        while timer.read() < next_target_time {
            // Do nothing
        }

        next_target_time = next_target_time.wrapping_add(target_interval_us);

        count += 1;

        if count % 24 == 0 {
            write!(serial, "Beat ").unwrap();
        }

        if count % 96 == 0 {
            write!(serial, "---- Bar ----\r\n").unwrap();
        }
        
    }
}

fn get_clock_pulse_for_bpm(bpm: u32) -> u32 {
    let us_per_minute = 1_000_000 * 60;
    let pulses_per_minute = bpm * 24;
    us_per_minute / pulses_per_minute
}