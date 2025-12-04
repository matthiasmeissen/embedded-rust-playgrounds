#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};
use microbit::{board, display::blocking::Display, hal::Timer};
use rtt_target::{rtt_init_print, rprintln};
use panic_halt as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut pins = board.display_pins;

    pins.row1.set_high().unwrap();
    pins.col1.set_high().unwrap();
    pins.col2.set_high().unwrap();

    let mut samples: u32 = 0;
    let sample_rate: u32 = 1000;
    let bpm = 120;

    let beat_length = 60.0 / bpm as f32;
    let quarter_length = beat_length / 4.0;

    let mut prev_beat_phasor = 0.0;
    let mut prev_quarter_phasor = 0.0;

    rprintln!("Phasor running.");

    loop {
        // Wait for one ms (sample rate)
        timer.delay_ms(1u32);

        samples += 1;

        let seconds = samples as f32 / sample_rate as f32;

        let beat_seconds = seconds % beat_length;
        let quarter_seconds = seconds % quarter_length;
        
        let beat_phasor = beat_seconds / beat_length;
        let quarter_phasor = quarter_seconds / quarter_length;
        
        if prev_beat_phasor > beat_phasor {
            rprintln!("Beat");
            pins.col1.set_low().unwrap();
        } else {
            pins.col1.set_high().unwrap();
        }
        
        if prev_quarter_phasor > quarter_phasor {
            rprintln!("Quarter");
            pins.col2.set_low().unwrap();
        } else {
            pins.col2.set_high().unwrap();
        }

        prev_beat_phasor = beat_phasor;
        prev_quarter_phasor = quarter_phasor;
    }
}