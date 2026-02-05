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

static SHARED_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let Some(gpiote) = SHARED_GPIOTE.borrow(cs).borrow_mut().as_mut() {
            if gpiote.channel0().is_event_triggered() {
                rprintln!("Button A");
                gpiote.channel0().reset_events();
            }
            if gpiote.channel1().is_event_triggered() {
                rprintln!("Button B");
                gpiote.channel1().reset_events();
            }
        }
    })
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();

    let button_a = board.buttons.button_a.into_pullup_input();
    let button_b = board.buttons.button_b.into_pullup_input();

    let gpiote = Gpiote::new(board.GPIOTE);
    let channel0 = gpiote.channel0();
    channel0.input_pin(&button_a.degrade()).hi_to_lo().enable_interrupt();
    channel0.reset_events();
    let channel1 = gpiote.channel1();
    channel1.input_pin(&button_b.degrade()).hi_to_lo().enable_interrupt();
    channel0.reset_events();

    cortex_m::interrupt::free(|cs| {
        SHARED_GPIOTE.borrow(cs).replace(Some(gpiote));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::interrupt::GPIOTE);

    let mut hardware_rng = Rng::new(board.RNG);
    let mut prng = Prng::seeded(&mut hardware_rng);
    let random_number = prng.random_u32();
    rprintln!("Random number is: {}", random_number);

    let snake = Snake::new();
    rprintln!("Init             Snake is: {:?}", snake);


    loop {
        asm::wfi();
    }
}