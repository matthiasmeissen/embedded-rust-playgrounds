#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit::{
    board, 
    display::nonblocking::GreyscaleImage, 
    hal::Rng
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

mod game;

use crate::game::Game;

mod controls;
mod display;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();

    display::init_display(board.TIMER1, board.display_pins, board.CLOCK, board.RTC0);

    controls::init_buttons(board.GPIOTE, board.buttons);

    let mut hardware_rng = Rng::new(board.RNG);

    let game = Game::new(&mut hardware_rng);

    loop {
        asm::wfi();

        let current_turn = controls::get_turn(false);
        rprintln!("Turn is: {:?}", current_turn);

        let image = GreyscaleImage::new(&game.game_matrix(6, 5, 9));
        display::show_image(&image);
    }
}