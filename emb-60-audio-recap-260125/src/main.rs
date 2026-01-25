#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use libm;
use embedded_hal::{digital::OutputPin, delay::DelayNs};
use microbit::{
    board, hal::{Timer, gpiote::Gpiote, pac::interrupt}, pac::{self, TIMER0, rtc0::{COUNTER, counter}}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

type SpeakerType = microbit::hal::gpio::Pin<microbit::hal::gpio::Output<microbit::hal::gpio::PushPull>>;
type TimerType = Timer<TIMER0>;

static SHARED_SIREN: Mutex<RefCell<Option<Siren>>> = Mutex::new(RefCell::new(None));

struct Siren {
    speaker: SpeakerType,
    timer: TimerType,
    current_phase: bool,
    base_delay: u32,
    delay: u32,
    current_angle: f32,
    step_counter: u32,
}

impl Siren {
    fn new(speaker: SpeakerType, timer: TimerType) -> Self {
        Self { 
            speaker, 
            timer, 
            current_phase: false, 
            base_delay: 1_000_000 / 440 / 2, 
            delay: 0, 
            current_angle: 0.0,
            step_counter: 0 
        }
    }

    fn step(&mut self) {
        // Reset the interrupt
        self.timer.reset_event();

        // Disconnect interrupt after certain time
        self.disconnect();

        // Set output high or low depending on internat state
        if self.current_phase {
            self.speaker.set_high().unwrap();
        } else {
            self.speaker.set_low().unwrap();
        }

        // Flip State
        self.current_phase = !self.current_phase;

        // Modulate delay based on counter value added to base delay
        self.modulate_delay_time(0.002, 400.0);

        // Start timer with defined delay time
        self.timer.start(self.delay);
    }

    fn modulate_delay_time(&mut self, freq: f32, depth: f32) {
        self.current_angle += freq;

        if self.current_angle > 6.28318 {
            self.current_angle -= 6.28318;
        }

        let sin_value = libm::sinf(self.current_angle);

        let modulation_value = (sin_value + 1.0) * depth;

        self.delay = self.base_delay + modulation_value as u32;

        self.step_counter += 1;
    }

    fn disconnect(&mut self) {
        if self.step_counter > 10_000 {
            self.timer.disable_interrupt();
        }
    }
}

#[interrupt]
fn TIMER0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(siren) = SHARED_SIREN.borrow(cs).borrow_mut().as_mut() {
            siren.step();
        }
    })
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = board::Board::take().unwrap();
    let speaker: SpeakerType = board.speaker_pin.into_push_pull_output(microbit::hal::gpio::Level::Low).degrade();

    let mut timer: TimerType = Timer::new(board.TIMER0);
    timer.enable_interrupt();
    timer.start(1_000);

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