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


// I2C - Inter Integrated Circuit

// Needs two lines SCL (Clock) and SDA (Data)
// Uses a Controller - Target Model
// The Controller starts the communication to any number of targets

// Communications from Controller to Target goes:
// C:START(1)    C:TARGET_ADDRESS (7) + R/W(1)     T:ACKNOWLEDGE(1)    C:SEND_DATA(8)      T:ACK(1)        (SEND -> ACK multiple times)        C:STOP(1)


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();

    let mut i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let mut acc = [0u8];
    let mut mag = [0u8];

    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();
    i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();

    rprintln!("The accelerometer chip ID is: {:#b}", acc[0]);
    rprintln!("The magnetometer chip ID is: {:#b}", mag[0]);

    loop {
        wfi();
    }
}