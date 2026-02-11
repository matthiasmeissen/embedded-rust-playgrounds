use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use microbit::{
    board::Buttons, 
    hal::gpiote::Gpiote,
    pac::{self,interrupt},
};
use rtt_target::rprintln;

use crate::game::movement::Turn;

static SHARED_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
pub static SHARED_TURN: Mutex<RefCell<Option<Turn>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let Some(gpiote) = SHARED_GPIOTE.borrow(cs).borrow().as_ref() {
            let a_pressed = gpiote.channel0().is_event_triggered();
            let b_pressed = gpiote.channel1().is_event_triggered();

            let turn = match (a_pressed, b_pressed) {
                (true, false) => Turn::Left,
                (false, true) => Turn::Right,
                _ => Turn::None
            };

            SHARED_TURN.borrow(cs).replace(Some(turn));

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();
        }
    })
}

pub fn init_buttons(board_gpiote: pac::GPIOTE, buttons: Buttons) {
    let button_a = buttons.button_a.into_pullup_input().degrade();
    let button_b = buttons.button_b.into_pullup_input().degrade();

    let gpiote = Gpiote::new(board_gpiote);

    let channel0 = gpiote.channel0();
    channel0.input_pin(&button_a).hi_to_lo().enable_interrupt();
    channel0.reset_events();
    let channel1 = gpiote.channel1();
    channel1.input_pin(&button_b).hi_to_lo().enable_interrupt();
    channel1.reset_events();


    cortex_m::interrupt::free(|cs| {
        SHARED_GPIOTE.borrow(cs).replace(Some(gpiote));
        SHARED_TURN.borrow(cs).replace(Some(Turn::None));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::interrupt::GPIOTE);
}

pub fn get_turn(reset: bool) -> Turn {
    if reset {
        return Turn::None;
    }
    
    let mut turn = Turn::None;
    cortex_m::interrupt::free(|cs| {
        if let Some(inner_turn) = SHARED_TURN.borrow(cs).borrow().as_ref() {
            turn = *inner_turn;
        }
    });
    turn
}
