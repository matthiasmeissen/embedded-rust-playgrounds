#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{OutputPin, InputPin}};
use microbit::{board, hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut pins = board.display_pins;

    pins.row1.set_high().unwrap();
    pins.col1.set_high().unwrap();


    let mut button_a = board.buttons.button_a.into_pullup_input();

    let mut last_state = false;

    let mut light_led = false;

    loop {
        // Set current state to button pressed state (low is pressed)
        let current_state = button_a.is_low().unwrap();
        // Wait for 20ms
        timer.delay_ms(20);
        // Check if button is still pressed
        if current_state == button_a.is_low().unwrap() {
            // When still pressed check if last state is not current state
            if !last_state && current_state {
                // When so, set variable to toggle led
                light_led = !light_led;
            }
            // Set last state to current state
            last_state = current_state;   
        }
        // Light up led based on variable
        if light_led {
            pins.col1.set_low().unwrap();
        } else {
            pins.col1.set_high().unwrap();
        }
    }
}