#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{
    Board, hal::{Timer, gpiote::Gpiote, pac::interrupt}, pac::{self, TIMER0, rtc0::COUNTER}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // To access an LED pin you take the DisplayPin from the board
    let mut pins = board.display_pins;
    // You then set row1 to high and col1 to low to see the top left LED
    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();

    // In theory the same should work for the speaker
    // The question is how to get it the speaker pin
    // It seems to be disconnected by default, so lets turn it into some output type
    let mut speaker = board.speaker_pin.into_push_pull_output(microbit::hal::gpio::Level::Low);

    // We delay in ms, which is 1 / 1000 second
    // To get a desired freqency we divide 1000 / FREQ
    // And the divide it by two in the delay to get one complete cycle
    const PERIOD: u32 = 1000 / 220;
    const COUNTER: usize = 500;

    rprintln!("Start Speaker at 220HZ");

    for _ in 0..COUNTER {
        speaker.set_high().unwrap();
        timer.delay_ms(PERIOD / 2);
        speaker.set_low().unwrap();
        timer.delay_ms(PERIOD / 2);
    }

    loop {
        asm::wfi();
    }
}