#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use nrf52833_hal::{ Timer, gpio, pac, timer};
use panic_halt as _;

struct LedPins {
    row1: gpio::p0::P0_21<gpio::Output<gpio::PushPull>>,
    row2: gpio::p0::P0_22<gpio::Output<gpio::PushPull>>,
    row3: gpio::p0::P0_15<gpio::Output<gpio::PushPull>>,
    row4: gpio::p0::P0_24<gpio::Output<gpio::PushPull>>,
    row5: gpio::p0::P0_19<gpio::Output<gpio::PushPull>>,
    col1: gpio::p0::P0_28<gpio::Output<gpio::PushPull>>,
    col2: gpio::p0::P0_11<gpio::Output<gpio::PushPull>>,
    col3: gpio::p0::P0_31<gpio::Output<gpio::PushPull>>,
    col4: gpio::p1::P1_05<gpio::Output<gpio::PushPull>>,
    col5: gpio::p0::P0_30<gpio::Output<gpio::PushPull>>,
}

impl LedPins {
    fn init(p0: gpio::p0::Parts, p1: gpio::p1::Parts) -> Self {
        Self {
            row1: p0.p0_21.into_push_pull_output(gpio::Level::Low),
            row2: p0.p0_22.into_push_pull_output(gpio::Level::Low),
            row3: p0.p0_15.into_push_pull_output(gpio::Level::Low),
            row4: p0.p0_24.into_push_pull_output(gpio::Level::Low),
            row5: p0.p0_19.into_push_pull_output(gpio::Level::Low),

            col1: p0.p0_28.into_push_pull_output(gpio::Level::High),
            col2: p0.p0_11.into_push_pull_output(gpio::Level::High),
            col3: p0.p0_31.into_push_pull_output(gpio::Level::High),
            col4: p1.p1_05.into_push_pull_output(gpio::Level::High),
            col5: p0.p0_30.into_push_pull_output(gpio::Level::High),
        }
    }

    fn reset(&mut self) {
        self.row1.set_low().unwrap();
        self.row2.set_low().unwrap();
        self.row3.set_low().unwrap();
        self.row4.set_low().unwrap();
        self.row5.set_low().unwrap();

        self.col1.set_high().unwrap();
        self.col2.set_high().unwrap();
        self.col3.set_high().unwrap();
        self.col4.set_high().unwrap();
        self.col5.set_high().unwrap();
    }
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let p1 = gpio::p1::Parts::new(peripherals.P1);
    let mut timer0 = timer::Timer::new(peripherals.TIMER0);

    let mut led_pins = LedPins::init(p0, p1);

    let delay = 2;
    let mut index = 0;

    loop{
        match index {
            0 => left_top(&mut led_pins),
            1 => right_top(&mut led_pins),
            2 => right_bottom(&mut led_pins),
            3 => left_bottom(&mut led_pins),
            _ => (),
        }
        reset_pins_with_delay(&mut led_pins, &mut timer0, delay);

        index += 1;
        if index > 3 { index = 0 };
    }
}

fn reset_pins_with_delay(led_pins: &mut LedPins, timer0: &mut Timer<pac::TIMER0>, delay: u32) {
    timer0.delay_ms(delay);
    led_pins.reset();
    timer0.delay_ms(delay);
}

fn left_top(led_pins: &mut LedPins) {
    led_pins.row1.set_high().unwrap();
    led_pins.col1.set_low().unwrap();
}

fn right_top(led_pins: &mut LedPins) {
    led_pins.row1.set_high().unwrap();
    led_pins.col5.set_low().unwrap();
}

fn left_bottom(led_pins: &mut LedPins) {
    led_pins.row5.set_high().unwrap();
    led_pins.col1.set_low().unwrap();
}

fn right_bottom(led_pins: &mut LedPins) {
    led_pins.row5.set_high().unwrap();
    led_pins.col5.set_low().unwrap();
}