
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{InputPin}};
use microbit::{board, display::blocking::Display, hal::Timer};
use panic_halt as _;


struct Counter {
    val: u8,
}

impl Counter {
    fn init() -> Self {
        Self { val: 0 }
    }

    fn add_1(&mut self) {
        if self.val < 31 {
            self.val += 1;
        }
    }

    fn sub_1(&mut self) {
        if self.val > 0 {
            self.val -= 1;
        }
    }
}

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    let mut display_state: [[u8; 5]; 5] = [[0; 5]; 5];

    let mut counter = Counter::init();

    loop {
        let state_a = button_a.is_low().unwrap();
        timer.delay_ms(20);
        if state_a == button_a.is_low().unwrap() {
            counter.sub_1();
        }

        let state_b = button_b.is_low().unwrap();
        timer.delay_ms(20);
        if state_b == button_b.is_low().unwrap() {
            counter.add_1();
        }
    }
}
