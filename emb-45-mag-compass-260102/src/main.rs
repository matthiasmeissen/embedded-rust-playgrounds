
#![no_std]
#![no_main]

use core::i32;

use cortex_m_rt::entry;
use microbit::{
    board, display::{blocking::Display}, hal::{Timer, twim}, pac::twim0::frequency::FREQUENCY_A
};
use lsm303agr::{MagMode, MagOutputDataRate, Lsm303agr};
use rtt_target::{rtt_init_print, rprintln};
use libm::atan2f;
use panic_rtt_target as _;

mod calibrate;
use calibrate::Calibration;

mod sprites;
use sprites::*;

const CALIBRATION_DURATION: i32 = 200;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_mode_and_odr(&mut timer, MagMode::HighResolution, MagOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let mut calibrate = Calibration::new();
    let mut counter = 0;
    let mut current_sprite = S0;

    loop {
        if sensor.mag_status().unwrap().xyz_new_data() {
            let (x, y, z) = sensor.magnetic_field().unwrap().xyz_nt();

            if calibrate.is_calibrating {
                if counter < CALIBRATION_DURATION {
                    calibrate.update(x, y, z);
                    rprintln!("Calibrating: {:?}", calibrate);
                    counter += 1;
                } else {
                    calibrate.is_calibrating = false;
                    rprintln!("Calibration done.");
                    counter = 0;
                }
            } else {
                calibrate.calculate_offset(x, y, z);
                let angle = atan2f(calibrate.cy as f32, calibrate.cx as f32).to_degrees();
                rprintln!("Angle: {}", angle);

                current_sprite = match angle as i32 {
                    0..45 => S0,
                    45..90 => S7,
                    90..135 => S6,
                    135..180 => S5,
                    -180..-135 => S4,
                    -135..-90 => S3,
                    -90..-45 => S2,
                    -45..0 => S1,
                    _ => S0,
                }
            }
        }
        display.show(&mut timer, current_sprite, 20);
    }
}
