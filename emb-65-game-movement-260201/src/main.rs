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
use game::coords::Coords;
use game::rng::Prng;
use game::snake::Snake;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let mut hardware_rng = Rng::new(board.RNG);

    let mut prng = Prng::seeded(&mut hardware_rng);
    let random_number = prng.random_u32();
    rprintln!("Random number is: {}", random_number);

    let mut snake = Snake::new();
    rprintln!("Snake is: {:?}", snake);
    snake.step_grow(Coords::new(2, 3));
    rprintln!("Snake is: {:?}", snake);
    snake.turn_right();
    rprintln!("Snake is: {:?}", snake);

    loop {
        asm::wfi();
    }
}