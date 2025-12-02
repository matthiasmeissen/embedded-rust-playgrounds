#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{InputPin,OutputPin}};
use microbit::{board, hal::Timer};
use rtt_target::{rtt_init_print, rprintln};
use panic_halt as _;


// Real Time Transfer
// High speed bidirectional communication channel between embedded device and host computer

// How to install
// Add rtt library and provide critical section to deal with interrups
// `Cargo.toml`         rtt-target crate
// `Cargo.toml`         cortex-m = { version = "0.7.7", features = ["critical-section-single-core"] }
// Tell the cargo embed command to use rtt when flashing to device
// `Embed.toml`         [default.rtt]
//                      enabled = true

// Usage
// First call rtt_init_print!(); to set up an rtt channel
// Then use rprintln!(); whenever you want to print a value to the console


#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Program started");

    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut pins = board.display_pins;
    let mut button_a = board.buttons.button_a.into_pullup_input();

    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();

    rprintln!("LEDs initialized, entering loop");

    let mut count = 0;
    let mut last_state = false;
    let mut light_led = false;

    loop {
        let current_state = button_a.is_low().unwrap();
        timer.delay_ms(20);

        // Debounce (only execute code when state is stable)
        if current_state == button_a.is_low().unwrap() {
            // Detect rising edge (button press)
            // When current state low and last is high
            if current_state && !last_state {
                count += 1;
                light_led = !light_led;
                
                if light_led {
                    rprintln!("Led on.      Iteration: {}", count);
                } else {
                    rprintln!("Led off.     Iteration: {}", count);
                }
            }
            last_state = current_state;
        }

        if light_led  {
            pins.col1.set_low().unwrap();
        } else {
            pins.col1.set_high().unwrap();
        }
    }
}