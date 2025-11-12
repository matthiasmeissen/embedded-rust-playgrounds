// Configure to use no_std which means we only can use the core libraray since we have no os available
#![no_std]
// This means that we do not have a main function that runs within the os, we are the only program that is on the device
#![no_main]

// To still specify where the program should start we use the entry macro from a crate for the chip we use
use cortex_m_rt::entry;
// The board specific hal acts as an abstraction layer to access the elements of the chip
use nrf52833_hal::{ gpio, pac, timer };
// The embedded hal crate includes traits or common functions that can be performed on embedded devices
use embedded_hal::{digital::OutputPin, delay::DelayNs};
// The panic halt crate is a way to define what shoudl happen when the program panics, since we cant just return to the os here
use panic_halt as _;

// The entry function returns a ! which means that the program is intended to run forever, which is also the reason for the loop inside
#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let mut timer0 = timer::Timer::new(peripherals.TIMER0);

    // Pins can be configured into different modes

    // Input - Can read electrical signals
    // Examples
    // .into_floating_input() -> Pin is not connected to VCC or GND internally.
    // .into_pulldown_input() -> Internal resistor pulls pin HIGH (3.3V) by default, Pressing button connected to GND makes pin read LOW
    // .into_pullup_input() -> Internal resistor pulls pin LOW (0V) by default, External signal to 3.3V makes pin read HIGH

    // Output - Can drive electrical signals
    // Examples
    // .into_push_pull_output() -> Can actively drive high (3V3) or pull low (0V)
    // .into_open_drain_output() -> Can only actively pull low, needs pullup resistir fo high

    // When we call the .into_push_pull_output() method on a pin we need to give it an initial stat argument
    let mut row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let mut col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    let mut col3 = p0.p0_31.into_push_pull_output(gpio::Level::Low);
    let mut row3 = p0.p0_15.into_push_pull_output(gpio::Level::High);

    
    loop {
        // We can use methods from the embedded hal crate which makes setting pin states simpler
        col1.set_low().unwrap();
        // This also applies to the timer
        timer0.delay_ms(200);
        col1.set_high().unwrap();
        col3.set_low().unwrap();
        timer0.delay_ms(200);
        row3.set_high().unwrap();
        row1.set_high().unwrap();
        timer0.delay_ms(200);
        col1.set_low().unwrap();
        col3.set_high().unwrap();
        timer0.delay_ms(200);
        row1.set_low().unwrap();
    }
}