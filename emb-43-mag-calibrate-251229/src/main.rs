#![no_std]
#![no_main]

use core::i32;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs};
use microbit::{
    board, 
    hal::{Timer, twim}, 
    pac::twim0::{frequency::FREQUENCY_A},
};
use lsm303agr::{MagMode, MagOutputDataRate, Lsm303agr};
use rtt_target::{rtt_init_print, rprintln};
use libm::sqrtf;
use panic_rtt_target as _;

#[derive(Debug, PartialEq, Clone)]
struct Calibration {
    x: i32,
    x_min: i32,
    x_max: i32,
    y: i32,
    y_min: i32,
    y_max: i32,
    z: i32,
    z_min: i32,
    z_max: i32,
    is_calibrating: bool,
}

impl Calibration {
    fn new() -> Self {
        Self { x: 0, x_min: i32::MAX, x_max: i32::MIN, y: 0, y_min: i32::MAX, y_max: i32::MIN, z: 0, z_min: i32::MAX, z_max: i32::MIN, is_calibrating: true }
    }

    fn update(&mut self, x: i32, y: i32, z: i32) {
        if x > self.x_max { self.x_max = x };
        if x < self.x_min { self.x_min = x };

        if y > self.y_max { self.y_max = y };
        if y < self.y_min { self.y_min = y };

        if z > self.z_max { self.z_max = z };
        if z < self.z_min { self.z_min = z };

        self.x = (self.x_min + self.x_max) / 2;
        self.y = (self.y_min + self.y_max) / 2;
        self.z = (self.z_min + self.z_max) / 2;
    }

    fn get_average(&mut self) -> (i32, i32, i32) {   
        (self.x, self.y, self.z)
    }
}

const CALIBRATION_DURATION: i32 = 400;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut timer = Timer::new(board.TIMER0);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_mode_and_odr(&mut timer, MagMode::HighResolution, MagOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let mut calibration = Calibration::new();
    rprintln!("{:?}", calibration);

    let mut prev_calibration = Calibration::new();
    let mut counter = 0;

    loop {
        if sensor.mag_status().unwrap().xyz_new_data() {
            let (x, y, z) = sensor.magnetic_field().unwrap().xyz_nt();

            if calibration.is_calibrating {
                if counter < CALIBRATION_DURATION  {
                    calibration.update(x, y, z);
                    rprintln!("{:?}", calibration);
                    counter += 1;
                } else {
                    rprintln!("Calibration done: X:{}, Y: {}, Z: {}", calibration.x, calibration.y, calibration.z);
                    calibration.is_calibrating = false;
                }
            } else {
                let (ox, oy, oz) = calibration.get_average();
                let (cx, cy, cz) = (x - ox, y - oy, z - oz);
                let (cx, cy, cz) = (cx as f32, cy as f32, cz as f32); 

                let magnitude = sqrtf(cx * cx + cy * cy + cz * cz);
                rprintln!("Calibrated magnitude: {} mg", magnitude / 100.0);
            }
        }
    }
}