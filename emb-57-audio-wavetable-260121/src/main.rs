#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{
    Board, hal::{Timer, gpiote::Gpiote, pac::interrupt}, pac::{self, TIMER0, rtc0::COUNTER}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

// We can store any waveform in an array
// But we wont be able to use it on the speaker, since it only can be low or high and not in between
// We could try using the table to modulate the delay times, which in turn might create the wave
const TRIANGLEWAVE: [f32; 16] = [0.0, 0.25, 0.5, 0.75, 1.0, 0.75, 0.5, 0.25, 0.0, -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut speaker = board.speaker_pin.into_push_pull_output(microbit::hal::gpio::Level::Low);


    // Delay in ms
    // 1 sec = 1000ms
    // 1 Hz = 1 Cycle / second
    // Divide by 2 for delays that show only half cycle (square)
    // DelayFullCycle(ms) = 1000 / Freq(Hz)             1000 / 440      = 2.27
    // DelayHalfCycle(ms) = 1000 / Freq(Hz) / 2         1000 / 440 / 2  = 1.135   

    // This approach will not work since sounding will give us just 1 with the u32 type
    // To avoid that we need to go into microseconds (us)
    const DELAYHALF: u32 = 1_000_000 / 440 / 2;

    rprintln!("Playing square for {} microseconds", DELAYHALF);

    // Play square wave for 500 * 2.27 = 1135 ms = 1.2 sec
    // for _ in 0..500 {
    //     speaker.set_high().unwrap();
    //     // Timer delay accepts u32, so it will not include digits, which will get unprecise
    //     timer.delay_us(DELAYHALF);
    //     speaker.set_low().unwrap();
    //     timer.delay_us(DELAYHALF);
    // }

    for i in 0..500 {
        // Lets loop through the items of the array and offset each by one to get only positive values
        // Might not work since we use u32 for delay
        let modulation = ((TRIANGLEWAVE[i % 16] + 1.0) * 1_000.0) as u32;
        speaker.set_high().unwrap();
        // Lets switch to ns to make use of the modulation properly
        timer.delay_us(DELAYHALF + modulation);
        speaker.set_low().unwrap();
        timer.delay_us(DELAYHALF + modulation);
    }

    loop {
        asm::wfi();
    }
}