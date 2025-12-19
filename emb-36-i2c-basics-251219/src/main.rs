
#![no_std]
#![no_main]

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use embedded_hal::i2c::I2c;
use microbit::{
    board, 
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();

    let mut i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let mut acc = [0u8];

    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();

    rprintln!("The accelerometer chip ID is: {:#b}", acc[0]);

    loop {
        wfi();
    }
}
