use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use microbit::{
    board::Buttons, 
    hal::gpiote::Gpiote,
    pac::{self,interrupt},
};
use rtt_target::rprintln;

static SHARED_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let Some(gpiote) = SHARED_GPIOTE.borrow(cs).borrow_mut().as_mut() {
            if gpiote.channel0().is_event_triggered() {
                rprintln!("Button A");
                gpiote.channel0().reset_events();
            }
            if gpiote.channel1().is_event_triggered() {
                rprintln!("Button B");
                gpiote.channel1().reset_events();
            }
        }
    })
}

pub fn init_buttons(board_gpiote: pac::GPIOTE, buttons: Buttons) {
    let button_a = buttons.button_a.into_pullup_input().degrade();
    let button_b = buttons.button_b.into_pullup_input().degrade();

    let gpiote = Gpiote::new(board_gpiote);

    let channel0 = gpiote.channel0();
    channel0.input_pin(&button_a).hi_to_lo().enable_interrupt();
    channel0.reset_events();
    let channel1 = gpiote.channel1();
    channel1.input_pin(&button_b).hi_to_lo().enable_interrupt();
    channel1.reset_events();


    cortex_m::interrupt::free(|cs| {
        SHARED_GPIOTE.borrow(cs).replace(Some(gpiote));
    });

    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::interrupt::GPIOTE);
}