#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};
use microbit::{board::{self, Buttons}, display::blocking::Display, hal::Timer, pac::{p0::out, rtc0::counter}};
use panic_halt as _;


struct BinaryCounter {
    val: u8,
}

impl BinaryCounter {
    fn init() -> Self {
        Self { val: 0b00000 }
    }

    fn add_1(&mut self) {
        if self.val < 0b11111 {
            self.val += 0b00001;
        }
    }

    fn sub_1(&mut self) {
        if self.val > 0b00000 {
            self.val -= 0b00001;
        }
    }
}

struct DebounceButton<T: InputPin> {
    button: T,
    last_state: bool,
    current_state: bool,
}

impl<T: InputPin> DebounceButton<T> {
    fn new(input_button: T) -> Self {
        Self {
            button: input_button,
            last_state: false,
            current_state: false
        }
    }

    fn debounce<A: DelayNs, C: FnMut() -> ()>(&mut self, timer: &mut A, mut f: C) {
        self.current_state = self.button.is_low().unwrap();
        timer.delay_ms(20);
        if self.current_state == self.button.is_low().unwrap() {
            // When button is still pressed after 20ms
            // Check if last state was not pressed and this one is
            if !self.last_state && self.current_state {
                f();
            }
            // Set last state to current state
            self.last_state = self.current_state;
        }
    }

    fn read<C: FnMut() -> ()>(&mut self, mut f: C) {
        self.current_state = self.button.is_low().unwrap();

        if !self.last_state && self.current_state {
            f();
        }

        self.last_state = self.current_state;
    }
}

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut debounce_button_a = DebounceButton::new(board.buttons.button_a.into_pullup_input());
    let mut debounce_button_b = DebounceButton::new(board.buttons.button_b.into_pullup_input());

    let mut display_state: [[u8; 5]; 5] = [[0; 5]; 5];

    let mut counter = BinaryCounter::init();

    loop {
        debounce_button_a.read(|| {counter.sub_1();});
        debounce_button_b.read(|| {counter.add_1();});

        display_state[4] = binary_to_array(counter.val);

        display.show(&mut timer, display_state, 20);
    }
}

fn binary_to_array(input: u8) -> [u8; 5] {
    let mut output = [0; 5];

    for i in 0..5 {
        let shift = 4 - i;
        let position = input >> shift;
        let val = position & 1;
        output[i] = val;
    };

    output
}