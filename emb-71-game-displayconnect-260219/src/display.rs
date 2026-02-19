
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

fn cross_image(inner_brightness: u8, outer_brightness: u8) -> GreyscaleImage {
    let b = inner_brightness;
    let c = outer_brightness;
    GreyscaleImage::new(&[
        [b, b, 0, b, b],
        [b, 7, 0, 7, b],
        [c, c, 7, c, c],
        [b, 7, 0, 7, b],
        [b, b, 0, b, b],
    ])
}

fn right_arrow_image(inner_brightness: u8) -> GreyscaleImage {
    let b = inner_brightness;
    GreyscaleImage::new(&[
        [0, 0, b, 0, 0],
        [0, 0, 0, b, 0],
        [7, 7, 7, 7, 7],
        [0, 0, 0, b, 0],
        [0, 0, b, 0, 0],
    ])
}

fn left_arrow_image(inner_brightness: u8) -> GreyscaleImage {
    let b = inner_brightness;
    GreyscaleImage::new(&[
        [0, 0, b, 0, 0],
        [0, b, 0, 0, 0],
        [7, 7, 7, 7, 7],
        [0, b, 0, 0, 0],
        [0, 0, b, 0, 0],
    ])
}

#[derive(Clone, Copy)]
pub enum ImageState {
    Cross,
    Right,
    Left,
}

pub static SHARED_DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));
pub static SHARED_TIMER: Mutex<RefCell<Option<Rtc<RTC0>>>> = Mutex::new(RefCell::new(None));
pub static SHARED_IMAGE_STATE: Mutex<RefCell<Option<ImageState>>> = Mutex::new(RefCell::new(None));

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
        SHARED_IMAGE_STATE.borrow(cs).replace(Some(ImageState::Cross));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::TIMER1) };
    unsafe { pac::NVIC::unmask(pac::interrupt::RTC0) };
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

#[interrupt]
fn RTC0() {
    static mut STEP: u8 = 0;
    let mut local_image_state = ImageState::Cross;

    cortex_m::interrupt::free(|cs| {
        if let Some(rtc) = SHARED_TIMER.borrow(cs).borrow_mut().as_mut() {
            rtc.reset_event(rtc::RtcInterrupt::Tick);
        }

        if let Some(state) = SHARED_IMAGE_STATE.borrow(cs).borrow().as_ref() {
            local_image_state = *state;
        }
    });

    let inner_brightness = match *STEP {
        0..=8 => 9 - *STEP,
        9..=12 => 0,
        _ => unreachable!(),
    };

    let outer_brightness = match *STEP {
        0..=4 => 0,
        5..=12 => 9 - (*STEP  / 3),
        _ => unreachable!(),
    };

    match local_image_state {
        ImageState::Cross => show_image(&cross_image(inner_brightness, outer_brightness)),
        ImageState::Right => show_image(&right_arrow_image(inner_brightness)),
        ImageState::Left => show_image(&left_arrow_image(inner_brightness)),
    }

    *STEP += 1;
    if *STEP == 13 {
        *STEP = 0
    };
}
