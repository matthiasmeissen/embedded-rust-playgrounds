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

// This uses the driver crate for an lsm303agr chip
// This chip is a ultra low power 3D accelerometer and 3D magnetometer

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    sensor.init().unwrap();

    sensor.set_accel_mode_and_odr(&mut timer, AccelMode::Normal, AccelOutputDataRate::Hz50).unwrap();

    rprintln!("Entering loop");

    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let data = sensor.acceleration().unwrap();
            //rprintln!("Acceleration: x {}, y {}, z {}", data.x_mg(), data.y_mg(), data.z_mg());
            let x = data.x_mg();
            let y = data.y_mg();
            let z = data.z_mg();

            print_bars([x, y, z]);
        }
    }
}

fn value_bar(input: i32) -> &'static str {
    let output: &str = if input < -1000 {
        "|________" 
    } else if input < -750 {
        "_|_______"
    } else if input < -500 {
        "__|______"
    } else if input < -250 {
        "___|_____"
    } else if input < 0 {
        "____|____"
    } else if input < 250 {
        "_____|___"
    } else if input < 500 {
        "______|__"
    } else if input < 750 {
        "_______|_"
    } else {
        "________|"
    };
    output
}

fn print_bars([x, y, z]: [i32; 3]) {
    rprintln!("X:{} Y:{} Z:{}", value_bar(x), value_bar(y), value_bar(z));
}