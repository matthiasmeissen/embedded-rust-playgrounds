
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
use tiny_led_matrix::Render;

pub static SHARED_DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));

pub fn init_display(timer: pac::TIMER1, pins: DisplayPins, clock: pac::CLOCK, rtc: pac::RTC0) {
    Clocks::new(clock).start_lfclk();

    let display = Display::new(timer, pins);

    cortex_m::interrupt::free(|cs| {
        SHARED_DISPLAY.borrow(cs).replace(Some(display));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::TIMER1) };
}

pub fn show_image(image: &impl Render) {
    cortex_m::interrupt::free(|cs| {
        if let Some(display) = SHARED_DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.show(image);
        }
    })
}

pub fn clear_screen() {
    cortex_m::interrupt::free(|cs| {
        if let Some(display) = SHARED_DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.clear();
        }
    })
}

#[interrupt]
fn TIMER1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(display) = SHARED_DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    })
}
