#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_hal::{gpio, pac, timer};
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let mut timer0 = timer::Timer::new(peripherals.TIMER0);

    let _row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let _col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    let _col3 = p0.p0_31.into_push_pull_output(gpio::Level::Low);
    let mut row3 = p0.p0_15.into_push_pull_output(gpio::Level::High);

    loop{
        timer0.delay_ms(200);
        row3.set_high().unwrap();
        timer0.delay_ms(200);
        row3.set_low().unwrap();
    }
}