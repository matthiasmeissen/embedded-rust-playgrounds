#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    board, 
    display::nonblocking::{BitImage, GreyscaleImage}, 
    hal::{Rng, Timer}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

mod game;

use crate::game::{Game, movement::GameStatus};

mod controls;
mod display;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0).into_periodic();
    let mut hardware_rng = Rng::new(board.RNG);
    let mut game = Game::new(&mut hardware_rng);

    display::init_display(board.TIMER1, board.display_pins, board.CLOCK, board.RTC0);
    controls::init_buttons(board.GPIOTE, board.buttons);

    loop {
        loop {
            let image = GreyscaleImage::new(&game.game_matrix(6, 3, 9));
            display::show_image(&image);
            timer.delay_ms(game.step_len_ms());
            
            match game.status {
                GameStatus::Ongoing => game.step(controls::get_turn(true)),
                _ => {
                    for _ in 0..3 {
                        display::clear_screen();
                        timer.delay_ms(200);
                        display::show_image(&image);
                        timer.delay_ms(200);
                    }
                    display::clear_screen();
                    display::show_image(&BitImage::new(&game.score_matrix()));
                    timer.delay_ms(200);
                    break;
                }
            }
        }
        game.reset();
    }
}