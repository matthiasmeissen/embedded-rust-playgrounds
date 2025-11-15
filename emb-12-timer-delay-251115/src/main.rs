#![no_std]
#![no_main]

// The cortex_m crate includes asm, which lets us use assembly instructions directly
// The nop instruction means do nothing
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{board, gpio::DisplayPins, hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut pins = board.display_pins;
    let mut timer0 = Timer::new(board.TIMER0);

    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();

    loop {
        better_delay(&mut pins, &mut timer0);
    }
}

// This function does nothing for 4million times, which equals roughly half a second
// Note that this only works when you run cargo embed --release
// Without the release part the code is not optimized and the wait() function will take far longer to execute
fn wait() {
    for _ in 0..4_000_000 {
        nop();
    }
}

// This is a far from ideal solution to implement a delay
// It is inaccurate and very hard to set to an exact value
// It varies for every chip since it is dependent on processing power
// It draws a lot of power since it is compultational intensive
// It is blocking, there can be no other task executed whiel this is running
fn unelegant_delay(pins: &mut DisplayPins) {
    wait();
    pins.col1.set_high().unwrap();
    wait();
    pins.col1.set_low().unwrap();
}

// A better way to implement a delay is to use one of teh chips timers
// This timer gives a very precise measurement
// It uses a periphery, so while it is still part of the chip, it is not part of the cpu
// Which means it is running independently and in theory can be non blocking
// In this specific case the .delay_ms() method is using a blocking abstraction
// This means it uses a loop that repeatedly checks if the timer is done yet, which is still very efficient
fn better_delay(pins: &mut DisplayPins, timer0: &mut Timer<microbit::pac::TIMER0>) {
    timer0.delay_ms(400);
    pins.col1.set_high().unwrap();
    timer0.delay_ms(400);
    pins.col1.set_low().unwrap();
}