#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit::{
    board, display::nonblocking::{Display, GreyscaleImage}, hal::{Rng, Timer, clocks::Clocks}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

mod game;
use game::rng::Prng;
use game::snake::Snake;

use crate::{display::{ImageState, SHARED_IMAGE_STATE}, game::movement::Turn};

mod controls;
mod display;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();

    display::init_display(board.TIMER1, board.display_pins, board.CLOCK, board.RTC0);

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
        let image_state = match current_turn {
            Turn::None => ImageState::Cross,
            Turn::Right => ImageState::Right,
            Turn::Left => ImageState::Left,
        };
        rprintln!("Turn is: {:?}", current_turn);
        cortex_m::interrupt::free(|cs| {
            SHARED_IMAGE_STATE.borrow(cs).replace(Some(image_state));
        })
    }
}