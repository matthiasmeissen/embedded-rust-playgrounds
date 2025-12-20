
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

const ACC_ADDR: u8 = 0b0011001;
const ACC_ID_REG: u8 = 0x0f;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();

    // This sets up the i2c channel with the TWIM (Two wire interface master / controller)
    let mut i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let mut ACC_ID = [0u8];

    // This sets a communication to the ACC Adress in write read mode
    // It first writes the register which contains the ID of the accelerometer to it
    // The accelerometer target responds with the ID, which the controller reads

    // Write phase: tell accelerometer which register we want
    // C:Start → C:Addr+W → T:Ack → C:IdReg → T:Ack
    // Read phase: read back the data
    // → C:ReStart → C:Addr+R → T:Ack → T:ID → C:NAck → C:Stop
    i2c.write_read(ACC_ADDR, &[ACC_ID_REG], &mut ACC_ID).unwrap();

    rprintln!("The id of the accelerometer chip is: {:#b}", ACC_ID[0]);

    loop {
        wfi();
    }
}
