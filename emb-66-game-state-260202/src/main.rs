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
    let mut timer = Timer::new(board.TIMER0);

    let mut hardware_rng = Rng::new(board.RNG);
    rprintln!("Test");

    let mut prng = Prng::seeded(&mut hardware_rng);
    let random_number = prng.random_u32();
    rprintln!("Random number is: {}", random_number);

    let mut snake = Snake::new();
    rprintln!("Init             Snake is: {:?}", snake);

    let next_coord = Coords::new(2, 3);
    snake.move_snake(next_coord, true);
    rprintln!("Move Forward     Snake is: {:?}", snake);

    snake.turn_right();
    rprintln!("Turn Right       Snake is: {:?}", snake);

    let next_coord = Coords::new(3, 3);
    snake.move_snake(next_coord, true);
    //rprintln!("Move Forward     Snake is: {:?}", snake);

    let random_coord = Coords::random(&mut prng, Some(&snake.coord_set));
    rprintln!("Random Coord     {:?}", random_coord);

    loop {
        asm::wfi();
    }
}