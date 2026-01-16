#![no_std]
#![no_main]

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use microbit::{
    Board, 
    hal::{Timer, gpiote::Gpiote, pac::interrupt}, 
    pac::{self, TIMER0}
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

// GPIOTE (GPIO Task and Events) Connects to pins and fires interrup event when activity
// NVIC (Nested Vetored Interrupts Controller) Has collection to all ISR in memory and maps Interrupt events to them
// ISR (Interrupt Service Routine) The function that gets called for a specific interrupt

// The only way to share data between the main loop and an interrupt is through static variables
// In order to make those memory safe we need to use special types
// For complex data we use an Option, wrapped in a Ref Cell, wrapped in a Mutex
// The Mutex in embedded is special compared to desktop, it only opens when it gets a critical section as key, which ensures that all interrupts are disabled currently
// The RefCell is a technique to make a static variable be mutable (I do not know exactly how that works, but it can change contents during runtime)
// Wen need this, because before initialization, there is no Gpiote instance available, so we pass it in Option<Gpiote> with None at first
static SHARED_GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
// We also create a variable to hold the Timer
static SHARED_TIMER: Mutex<RefCell<Option<Timer<TIMER0>>>> = Mutex::new(RefCell::new(None));
// For simpler types, we can just use an Atomic which can be safely shared between threads, allowing us access without complex setup
static COUNTER: AtomicUsize = AtomicUsize::new(0);

// Create ISR function with special interrupts macro and exact name of GPIOTE peripheral, the function must have no inputs and no returns
#[interrupt]
fn GPIOTE() {
    // Inside the ISR we use the same technique as in main to access the shared variable
    cortex_m::interrupt::free(|cs| {
        // First we get access to the timer using the if let technique to go theorugh the Mutex and RefCell
        if let Some(timer) = SHARED_TIMER.borrow(cs).borrow_mut().as_mut() {
            // We read the current value of the timer
            let count = timer.read();
            // When the timer has finished we run this code
            if count == 0 {
                // To access an Atomic it is way easier, we just use the fetch_add() method on it to increase the value
                COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                // We start the timer again, this means we only add 1 to the Counter on the first call
                // All other calls that might occur during mechnaical movement of the button are irgnored
                // This technique is called debouncing and must be used to avoid unwanted inputs
                timer.start(200_000);
            } else {
                rprintln!("Trigger avoid");
            }
        }

        // We use an if let technque to access only the Some() result of an option, we get a mutable refernce to the gpiote here
        if let Some(gpiote) = SHARED_GPIOTE.borrow(cs).borrow_mut().as_mut() {
            // Having access to the peripheral, we can create a refernce to channel0
            let channel0 = gpiote.channel0();
            // We check if there is a trigger flag on channel0 and execute some code
            if channel0.is_event_triggered() {
                // We need to reset the event flag on channel0, otherwise the interrupt with constantly fire
                channel0.reset_events();
            }
        }
    })
}

#[entry]
fn main() -> ! {
    // Initialize RTT (Real Time Transfer) to allow debugging print to console
    rtt_init_print!();
    // Get the peripherals from the board
    let board = Board::take().unwrap();
    // Get button a pin and turn into input pullup (high is up and low is pressed)
    let button_a = board.buttons.button_a.into_pullup_input();

    // Get access to the GPIOTE peripheral
    let gpiote = Gpiote::new(board.GPIOTE);
    // Get access to channel0 from gpiote (has total of 8 that can watch one pin each)
    let channel0 = gpiote.channel0();
    // Configure channel0 to watch button a pin go from hi to low, also enable interrupts
    channel0.input_pin(&button_a.degrade()).hi_to_lo().enable_interrupt();
    // Reset all events that might have occured during setup to avoid unwanted triggers
    channel0.reset_events();

    // Create an instance to the TIMER0 peripheral
    let mut timer = Timer::new(board.TIMER0);
    // We enable interrupts on the timer, actually not smart when no ISR is present
    //timer.enable_interrupt();
    // We start the timer to run for a given number of ticks
    // It counts of to that number at a 1MHz rate, which means we wait for 200ms for that
    timer.start(200_000);

    // To access a Mutex and change its content we need the critical section
    // This is done by the free() method, which takes a closure that passes that key
    cortex_m::interrupt::free(|cs| {
        // We first borrow with critical section to open the Mutex
        // And then use the replace() method to put Some(gpiote) inside the RefCell
        SHARED_GPIOTE.borrow(cs).replace(Some(gpiote));
        // We also store a reference to the timer peripheral
        SHARED_TIMER.borrow(cs).replace(Some(timer));
    });

    // Tell the NVIC to unmask the GPIOTE to enable it, this is unsafe because it is not type safe, since we need to make sure that the ISR is available
    unsafe { pac::NVIC::unmask(pac::interrupt::GPIOTE) };
    // Clear all pending events that might have occured before to avoid unnecessary triggers
    pac::NVIC::unpend(pac::interrupt::GPIOTE);

    loop {
        // Give assembly instruction called wait for interrupt, this stops the cpu and only execute when interrupt event occurs
        asm::wfi();
        // To get the counter value we just call the load() method
        let counter_val = COUNTER.load(core::sync::atomic::Ordering::Relaxed);
        // We print in the main look to keep the word done in the ISR as low as possible
        rprintln!("{}", counter_val);
    }
}