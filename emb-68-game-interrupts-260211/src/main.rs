#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit::{board, hal::Rng};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

mod game;
use game::rng::Prng;
use game::snake::Snake;

mod controls;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();

    controls::init_buttons(board.GPIOTE, board.buttons);

    let mut hardware_rng = Rng::new(board.RNG);
    let mut prng = Prng::seeded(&mut hardware_rng);
    let random_number = prng.random_u32();
    rprintln!("Random number is: {}", random_number);

    let snake = Snake::new();
    rprintln!("Init             Snake is: {:?}", snake);

    loop {
        asm::wfi();

        let current_turn = controls::get_turn(false);
        rprintln!("Turn is: {:?}", current_turn);
    }
}