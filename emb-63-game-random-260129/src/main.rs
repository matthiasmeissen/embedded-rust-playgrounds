#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};
use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{board, hal::{Rng, Timer, gpiote::Gpiote, pac::interrupt}, pac::{self, TIMER0, rtc0::{COUNTER, counter}}};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

mod game;
use game::coords;
use game::rng;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let mut hardware_rng = Rng::new(board.RNG);

    let mut prng = rng::Prng::seeded(&mut hardware_rng);
    let random_number = prng.random_u32();
    rprintln!("Random number is: {}", random_number);

    let random_number = prng.random_u32();
    rprintln!("Random number is: {}", random_number);

    let coord = coords::Coords::new(0, 0);
    rprintln!("{:?}", coord);

    loop {
        asm::wfi();
    }
}