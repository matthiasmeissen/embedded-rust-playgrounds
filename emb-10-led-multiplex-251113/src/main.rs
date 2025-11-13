#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{Board, gpio::DisplayPins, hal::Timer};
use panic_halt as _;

// The led matrix on the microbit is using multiplexing
// This is a technique to reduces the number of pins needed to drive leds
// It does so by not connecting each leds anode to vcc and cathode to gnd
// But by connecting the anodes in each row to the same pin
// And by connecting the cathodes in each column to the same pin
// This reduces the total number of pins in an 5x5 matrix from 25 to only 10 pins

// However, this techniques comes with a drawback
// While it is easy to light up complete rows
// It is impossible to light up different patterns on multiple rows
// To avoid that the leds are turned on an off at a very high frequency
// While in each iteration a different configuration is lit up
// So that we perceive them as constanly on

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut pins = board.display_pins;
    let mut timer0 = Timer::new(board.TIMER0);

    // This will light up all leds in row1 and row2
    //try_patterns_on_two_rows(&mut pins);

    // This lets us determine the speed in which we switch between the two states
    // When it is fast enough we perceive both states at the same time
    let speed = 10;

    loop {
        pattern_row1(&mut pins);
        timer0.delay_ms(speed);
        pattern_row2(&mut pins);
        timer0.delay_ms(speed);
    }
}

fn try_patterns_on_two_rows(pins: &mut DisplayPins) {
    // Light up led 1, 3 and 5 in row 1
    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();
    pins.col3.set_low().unwrap();
    pins.col5.set_low().unwrap();

    // Light up led 2 and 4 in row 2
    pins.row2.set_high().unwrap();
    pins.col2.set_low().unwrap();
    pins.col4.set_low().unwrap();
}

fn pattern_row1(pins: &mut DisplayPins) {
    // Using the break before make principle to avoid potentially undesired pin states
    clear_all_pins(pins);

    // Light up led 1, 3 and 5 in row 1
    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();
    pins.col3.set_low().unwrap();
    pins.col5.set_low().unwrap();
}

fn pattern_row2(pins: &mut DisplayPins) {
    // Using the break before make principle to avoid potentially undesired pin states
    clear_all_pins(pins);

    // Light up led 2 and 4 in row 2
    pins.row2.set_high().unwrap();
    pins.col2.set_low().unwrap();
    pins.col4.set_low().unwrap();
}

fn clear_all_pins(pins: &mut DisplayPins) {
    // All rows to low
    pins.row1.set_low().unwrap();
    pins.row2.set_low().unwrap();
    pins.row3.set_low().unwrap();
    pins.row4.set_low().unwrap();
    pins.row5.set_low().unwrap();

    // All cols to high
    pins.col1.set_high().unwrap();
    pins.col2.set_high().unwrap();
    pins.col3.set_high().unwrap();
    pins.col4.set_high().unwrap();
    pins.col5.set_high().unwrap();
}