
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use microbit::hal::{self,pac};
use pac::{p0, p1};

#[entry]
fn main() -> ! {
    init();

    unsafe {
        const PORT_P0_OUT: u32 = 0x50000504;

        // Turn on the top row
        *(PORT_P0_OUT as *mut u32) |= 1 << 21;

        // Turn on the bottom row
        *(PORT_P0_OUT as *mut u32) |= 1 << 19;

        // Turn off the top row
        *(PORT_P0_OUT as *mut u32) &= !(1 << 21);

        // Turn off the bottom row
        *(PORT_P0_OUT as *mut u32) &= !(1 << 19);
    }

    loop {}
}

#[inline(never)]
fn init() -> (&'static p0::RegisterBlock, &'static p1::RegisterBlock) {
    rtt_target::rtt_init_print!();
    let device_periphs = pac::Peripherals::take().unwrap();
    
    // `display_pins!` initializes the display pins as outputs in push-pull mode
    let port0 = hal::gpio::p0::Parts::new(device_periphs.P0);
    let port1 = hal::gpio::p1::Parts::new(device_periphs.P1);
    let _display_pins = microbit::display_pins!(port0, port1);

    (unsafe { &*pac::P0::ptr() }, unsafe { &*pac::P1::ptr() })
}
