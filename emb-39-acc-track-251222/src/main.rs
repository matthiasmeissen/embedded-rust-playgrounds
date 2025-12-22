#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board, 
    hal::{Timer, twim}, 
    pac::twim0::frequency::FREQUENCY_A,
};
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

const THRESHOLD: i32 = 400;
const DURATION: i32 = 10;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut timer = Timer::new(board.TIMER0);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();

    sensor.set_accel_mode_and_odr(&mut timer, AccelMode::HighResolution, AccelOutputDataRate::Hz10).unwrap();
    sensor.set_accel_scale(lsm303agr::AccelScale::G8).unwrap();

    let mut max_x = 0;
    let mut measure = false;
    let mut count = 0;

    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let data = sensor.acceleration().unwrap();
            let acc_x = data.x_mg();
            let abs_x = acc_x.abs();

            if abs_x > THRESHOLD && !measure {
                rprintln!("Enough force, start measurement.");
                measure = true;
                max_x = abs_x
            }

            if measure {
                count += 1;
                if abs_x > max_x {
                    max_x = abs_x;
                }

                if count > DURATION {
                    rprintln!("Max was: {}", max_x);
                    max_x = 0;
                    count = 0;
                    measure = false;
                }
            }
        }
    }
}