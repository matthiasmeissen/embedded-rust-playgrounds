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

type TimerType = Timer<TIMER0, microbit::hal::timer::Periodic>;
type SpeakerType = microbit::hal::gpio::Pin<microbit::hal::gpio::Output<microbit::hal::gpio::PushPull>>;

static SHARED_SIREN: Mutex<RefCell<Option<Siren>>> = Mutex::new(RefCell::new(None));


struct Siren {
    speaker: SpeakerType,
    timer: TimerType,
    freq: u32,
    counter: u32,
}

impl Siren {
    fn new(speaker: SpeakerType, timer: TimerType) -> Self {
        Siren { 
            speaker: speaker, 
            timer: timer, 
            freq: 1_000_000 / 440,
            counter: 0,
        }
    }

    // Delay is in microseconds
    // 1 second is 1_000_000 us
    // We divide by two since we have a squarewave
    fn step(&mut self) {
        self.speaker.set_high().unwrap();
        self.timer.delay_us(self.freq / 2);
        self.speaker.set_low().unwrap();
        self.timer.delay_us(self.freq / 2);
    }

    fn change_delay(&mut self) {
        let modulate = self.counter % 400;
        self.freq = 1_000 + modulate;
    }
}

#[interrupt]
fn TIMER0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(siren) = SHARED_SIREN.borrow(cs).borrow_mut().as_mut() {
            siren.counter += 1;
            rprintln!("{}", siren.counter);
            if siren.counter >= 1000 {
                siren.timer.disable_interrupt();
            }
            siren.change_delay();
            siren.step();
            siren.timer.reset_event();
        }
    })
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let speaker: SpeakerType = board.speaker_pin.into_push_pull_output(microbit::hal::gpio::Level::Low).degrade();

    let mut timer: Timer<TIMER0> = Timer::new(board.TIMER0);
    timer.enable_interrupt();
    let mut timer: TimerType= timer.into_periodic();
    // Timer cycles 440 times a second
    timer.start(timer_interval_from_hz(440));

    let siren = Siren::new(speaker, timer);

    cortex_m::interrupt::free(|cs| {
        SHARED_SIREN.borrow(cs).replace(Some(siren));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::TIMER0) };
    pac::NVIC::unpend(pac::interrupt::TIMER0);

    loop {
        asm::wfi();
    }
}

fn timer_interval_from_hz(hz: u32) -> u32 {
    1_000_000 / hz
}