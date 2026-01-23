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

static SHARED_SPEAKER: Mutex<RefCell<Option<microbit::hal::gpio::Pin<microbit::hal::gpio::Output<microbit::hal::gpio::PushPull>>>>> = Mutex::new(RefCell::new(None));
static SHARED_TIMER: Mutex<RefCell<Option<Timer<TIMER0, microbit::hal::timer::Periodic>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIMER0() {
    static mut SPEAKER_SATE: bool = false;
    static mut COUNTER: u32 = 0;
    cortex_m::interrupt::free(|cs| {
        if let Some(speaker) = SHARED_SPEAKER.borrow(cs).borrow_mut().as_mut() {
            if *SPEAKER_SATE {
                speaker.set_high().unwrap();
            } else {
                speaker.set_low().unwrap();
            }
            *SPEAKER_SATE = !*SPEAKER_SATE;
        }

        if let Some(timer) = SHARED_TIMER.borrow(cs).borrow_mut().as_mut() {
            if *COUNTER > 200 {
                rprintln!("Done");
                timer.disable_interrupt();
            }
            timer.reset_event();
            *COUNTER += 1;
        }
    })
}

#[entry]
fn main()-> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let speaker: microbit::hal::gpio::Pin<microbit::hal::gpio::Output<microbit::hal::gpio::PushPull>> = board.speaker_pin.into_push_pull_output(microbit::hal::gpio::Level::Low).degrade();

    let mut timer = Timer::new(board.TIMER0);
    timer.enable_interrupt();
    let mut timer: Timer<TIMER0, microbit::hal::timer::Periodic> = timer.into_periodic();
    // The timer is hardwired to 1_000_000 ticks / second
    // So to get a frquency in Hz we need to divided by that number
    timer.start(get_timer_durationfrom_frequency(440));

    cortex_m::interrupt::free(|cs| {
        SHARED_SPEAKER.borrow(cs).replace(Some(speaker));
        SHARED_TIMER.borrow(cs).replace(Some(timer));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::TIMER0) };
    pac::NVIC::unpend(pac::interrupt::TIMER0);

    loop {
        asm::wfi();
    }
}

fn get_timer_durationfrom_frequency(freq: u32) -> u32 {
    1_000_000 / (freq * 2)
}