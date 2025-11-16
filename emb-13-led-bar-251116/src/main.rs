
#![no_std]
#![no_main]

// Defines entry point in no_main program
use cortex_m_rt::entry;
// Abstraction layer for common functions on embedded devices
use embedded_hal::{digital::OutputPin, delay::DelayNs};
// BSC that provides abstraction to make usage easier
use microbit::{board, gpio::DisplayPins, hal::Timer, pac};
// We have no os (so we use no_std with only core functions) but need way to specify what should happen on errors
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut pins = board.display_pins;
    let mut timer0 = Timer::new(board.TIMER0);

    loop {
        draw_row1_pattern(&mut pins, &mut timer0, [0, 1, 0, 0, 0]);
    }
}

fn reset_pins(pins: &mut DisplayPins) {
    pins.row1.set_low().unwrap();
    pins.row2.set_low().unwrap();
    pins.row3.set_low().unwrap();
    pins.row4.set_low().unwrap();
    pins.row5.set_low().unwrap();

    pins.col1.set_high().unwrap();
    pins.col2.set_high().unwrap();
    pins.col3.set_high().unwrap();
    pins.col4.set_high().unwrap();
    pins.col5.set_high().unwrap();
}

fn draw_row1_pattern(pins: &mut DisplayPins, timer0: &mut Timer<pac::TIMER0>, pattern: [usize; 5]) {
    for i in pattern {
        reset_pins(pins);
        if i == 1 {
            light_row1_led_at_index(pins, i);
        }
        timer0.delay_ms(200);
    }
}

fn light_row1_led_at_index(pins: &mut DisplayPins, index: usize) {
    pins.row1.set_high().unwrap();

    match index {
        0 => pins.col1.set_low().unwrap(),
        1 => pins.col2.set_low().unwrap(),
        2 => pins.col3.set_low().unwrap(),
        3 => pins.col4.set_low().unwrap(),
        4 => pins.col5.set_low().unwrap(),
        _ => (),
    }
}
