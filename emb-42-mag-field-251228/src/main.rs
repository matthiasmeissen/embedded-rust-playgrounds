#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board, 
    hal::{Timer, twim}, 
    pac::twim0::{frequency::FREQUENCY_A},
};
use lsm303agr::{MagMode, MagOutputDataRate, Lsm303agr};
use rtt_target::{rtt_init_print, rprintln};
use libm::sqrtf;
use panic_rtt_target as _;

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

    loop {
        if sensor.mag_status().unwrap().xyz_new_data() {
            let (x, y, z) = sensor.magnetic_field().unwrap().xyz_nt();
            let (x, y, z) = (x as f32, y as f32, z as f32);
            let magnitude = sqrtf(x * x + y * y + z * z);
            rprintln!("Magnitude is: {} mg", magnitude / 100.0);
        }
    }
}