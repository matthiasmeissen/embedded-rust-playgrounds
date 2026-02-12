
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use microbit::{
    display::nonblocking::{Display, GreyscaleImage}, 
    gpio::DisplayPins, 
    hal::{
        Rtc, Timer, clocks::Clocks, rtc
    }, 
    pac::{self, RTC0, TIMER1, interrupt}
};

fn heart_image(inner_brightness: u8) -> GreyscaleImage {
    let b = inner_brightness;
    GreyscaleImage::new(&[
        [0, 7, 0, 7, 0],
        [7, b, 7, b, 7],
        [7, b, b, b, 7],
        [0, 7, b, 7, 0],
        [0, 0, 7, 0, 0],
    ])
}

static SHARED_DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));
static SHARED_TIMER: Mutex<RefCell<Option<Rtc<RTC0>>>> = Mutex::new(RefCell::new(None));

pub fn init_display(timer: pac::TIMER1, pins: DisplayPins, clock: pac::CLOCK, rtc: pac::RTC0) {
    Clocks::new(clock).start_lfclk();
    
    let mut rtc0 = rtc::Rtc::new(rtc, 2047).unwrap();
    rtc0.enable_event(rtc::RtcInterrupt::Tick);
    rtc0.enable_interrupt(rtc::RtcInterrupt::Tick, None);
    rtc0.enable_counter();

    let display = Display::new(timer, pins);

    cortex_m::interrupt::free(|cs| {
        SHARED_DISPLAY.borrow(cs).replace(Some(display));
        SHARED_TIMER.borrow(cs).replace(Some(rtc0));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::TIMER1) };
    unsafe { pac::NVIC::unmask(pac::interrupt::RTC0) };
}

#[interrupt]
fn TIMER1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(display) = SHARED_DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    })
}

#[interrupt]
fn RTC0() {
    static mut STEP: u8 = 0;

    cortex_m::interrupt::free(|cs| {
        if let Some(rtc) = SHARED_TIMER.borrow(cs).borrow_mut().as_mut() {
            rtc.reset_event(rtc::RtcInterrupt::Tick);
        }
    });

    let inner_brightness = match *STEP {
        0..=8 => 9 - *STEP,
        9..=12 => 0,
        _ => unreachable!(),
    };

    cortex_m::interrupt::free(|cs| {
        if let Some(display) = SHARED_DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.show(&heart_image(inner_brightness));
        }
    });

    *STEP += 1;
    if *STEP == 13 {
        *STEP = 0
    };
}
