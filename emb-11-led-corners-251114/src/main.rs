
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use nrf52833_hal::{ gpio, pac, timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let p1 = gpio::p1::Parts::new(peripherals.P1);
    let mut timer0 = timer::Timer::new(peripherals.TIMER0);

    let mut row1 = p0.p0_21.into_push_pull_output(gpio::Level::Low);
    let mut row2 = p0.p0_22.into_push_pull_output(gpio::Level::High);
    let mut row3 = p0.p0_15.into_push_pull_output(gpio::Level::Low);
    let mut row4 = p0.p0_24.into_push_pull_output(gpio::Level::Low);
    let mut row5 = p0.p0_19.into_push_pull_output(gpio::Level::Low);

    let mut col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);
    let mut col2 = p0.p0_11.into_push_pull_output(gpio::Level::Low);
    let mut col3 = p0.p0_31.into_push_pull_output(gpio::Level::Low);
    let mut col4 = p1.p1_05.into_push_pull_output(gpio::Level::Low);
    let mut col5 = p0.p0_30.into_push_pull_output(gpio::Level::Low);

    loop{}
}
