#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{board, display::blocking::Display, hal::{Rng, Timer}};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut rng = Rng::new(board.RNG);

    let mut button_b = board.buttons.button_b.into_pullup_input();

    let mut lights = [[0; 5]; 5];
    let mut is_target = false;
    let mut score: u8 = 0b00000;

    loop {
        (lights, is_target) = set_random_led(&mut rng);

        let mut button_pressed = false;

        let speed = if score > 30 as u8 {
            10
        } else {
            40 - score as u8 * 2
        };

        for _ in 0..speed {
            display.show(&mut timer, lights, 10);

            if button_b.is_low().unwrap() {
                button_pressed = true;
                break;
            }
        }

        if button_pressed && is_target {
            increase_score(&mut score);

            lights = [[1;5];5];
            display.show(&mut timer, lights, 400);
            display.clear();
            timer.delay_ms(200);

            lights = show_score(&mut score);
            display.show(&mut timer, lights, 1000);
        }

        display.clear();
        timer.delay_ms(200);
    }
}

fn set_random_led(rng: &mut Rng) -> ([[u8; 5]; 5], bool) {
    let mut lights = [[0; 5]; 5];

    let x = rng.random_u8() % 5;
    let y = rng.random_u8() % 5;

    let is_target= x == 2 && y == 2;

    lights[x as usize][y as usize] = 1;
    (lights, is_target)
}

fn increase_score(score: &mut u8) {
    *score += 0b00001;
}

fn show_score(score: &mut u8) -> [[u8; 5]; 5] {
    let mut display = [[0;5];5];
    display[0] = binary_to_row(score);
    display
}

fn binary_to_row(input: &mut u8) -> [u8; 5] {
    let mut row = [0; 5];
    for i in 0..5 {
        let shift = 4 - i;
        let position = *input >> shift;
        let val = position & 1;
        row[i] = val;
    };
    row
}