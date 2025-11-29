# Embedded Rust Playgrounds

A place to learn Rust for embedded devices.

**Prerequisites**: Completed 99 days of Rust fundamentals  
**Hardware Required**: BBC micro:bit v2 (~$20 USD)  
**Time Commitment**: ~30 minutes per day  
**Primary Resource**: [Discovery MB2 Book](https://docs.rust-embedded.org/discovery-mb2/)  
**Secondary Resource**: [The Embedded Rust Book](https://docs.rust-embedded.org/book/)  

---

## Phase 0: Foundation & Setup (Days 1-5)

### ✅ Day 1: Setup & Flash Your First Program!
- **Topic**: Installing the Rust embedded toolchain and immediately flashing a blinking LED to your micro:bit.
- **Exercise**: Install prerequisites: `rustup target add thumbv7em-none-eabihf` and `cargo install probe-rs-tools --features cli`. Clone the Discovery MB2 repo, connect your micro:bit, navigate to the `src/05-meet-your-software` directory, and run `cargo embed --release`. Watch your LED blink!
- **Hint**: If `cargo embed` fails, try `probe-rs list` to verify your board is detected. Make sure your micro:bit v2 is connected via USB.
- **Book**: Discovery MB2 - Chapter 3 (Setup) & Chapter 5 (Flash It)

### ✅ Day 2: Understanding What Just Happened
-  **Topic**: Breaking down the blinking LED program - what makes embedded Rust different from desktop Rust.
-  **Exercise**: Open the `main.rs` file you just flashed. Identify these key differences: `#![no_std]`, `#![no_main]`, the `#[entry]` attribute, and the infinite loop. Try changing the delay duration and reflash.
-  **Hint**: `no_std` means no standard library - no `Vec`, no `String`, no heap allocation! The `#[entry]` macro marks where your program starts (not `fn main()`).
-  **Book**: Discovery MB2 - Chapter 5: Meet Your Software | Embedded Rust Book - Chapter 4

### ✅ Day 3: Your First Code Change
- **Topic**: Modifying the LED blink pattern and understanding basic GPIO output.
- **Exercise**: Change the blink pattern to: on for 1 second, off for 1 second, on for 200ms, off for 200ms (repeating). Experiment with different patterns.
- **Hint**: Look for `board.display_pins` and the `.set_high()` / `.set_low()` methods. The delay is controlled by `timer.delay_ms()`.
- **Book**: Discovery MB2 - Chapter 5

### ✅ Day 4: Meet Your Hardware
- **Topic**: Understanding the BBC micro:bit v2 components and capabilities.
- **Exercise**: With your micro:bit in hand, physically identify: the nRF52833 chip (the large black square), the 5x5 LED matrix, buttons A and B, the USB connector, and the edge connector pins. Read Chapter 4 to understand what each does.
- **Hint**: The nRF52833 is an ARM Cortex-M4F processor (the 'F' means floating-point support) with 512KB Flash and 128KB RAM.
- **Book**: Discovery MB2 - Chapter 4: Meet Your Hardware

### ✅ Day 5: Project Structure Deep Dive
- **Topic**: Understanding the anatomy of an embedded Rust project - Cargo.toml, memory.x, and Embed.toml.
- **Exercise**: Examine your project's `Cargo.toml` - identify the key dependencies: `cortex-m`, `cortex-m-rt` (runtime), `nrf52833-hal` (hardware abstraction layer), and `panic-halt`. Read what each one provides.
- **Hint**: `cortex-m-rt` provides the startup code and interrupt vector table. `panic-halt` defines what happens when your program panics (it just halts).
- **Book**: Discovery MB2 - Chapter 5 | Embedded Rust Book - Chapter 5

---

## Phase 1: Basics & LED Control (Days 6-20)

### ✅ Day 6: Understanding `no_std`
- **Topic**: Why embedded systems don't use the standard library and what `core` provides.
- **Exercise**: Create a minimal `no_std` program. Try using a `Vec` and observe the compiler error. Replace it with a fixed-size array.
- **Hint**: You can't allocate memory dynamically without `std`!
- **Book**: Embedded Rust Book - Chapter 4: A `no_std` Application

### ✅ Day 7: Memory Layout Basics
- **Topic**: Understanding Flash (program storage) vs RAM (runtime data) and the `.text`, `.data`, `.bss` sections.
- **Exercise**: Use `cargo size` to inspect your binary's memory sections. Note which section is largest.
- **Hint**: Flash memory is non-volatile; RAM is volatile but faster.
- **Book**: Embedded Rust Book - Chapter 5: Memory Layout

### ✅ Day 8: Hello, LED!
- **Topic**: Controlling a single LED on the micro:bit's LED matrix using GPIO.
- **Exercise**: Light up the center LED (row 3, column 3) of the 5x5 matrix. Keep it on for 2 seconds, then turn it off.
- **Hint**: The LED matrix is multiplexed - you control rows and columns separately.
- **Book**: Discovery MB2 - Chapter 6: Hello World

### ✅ Day 9: GPIO Pin Configuration
- **Topic**: Understanding GPIO modes: input, output, push-pull, open-drain.
- **Exercise**: Configure pin P0.21 (LED row 1) as an output. Toggle it on and off 10 times with 500ms delays.
- **Hint**: Use `into_push_pull_output()` to configure a pin for driving LEDs.
- **Book**: Discovery MB2 - Chapter 7: GPIO

### ✅ Day 10: LED Matrix Rows
- **Topic**: Understanding row scanning in the LED matrix and how multiplexing works.
- **Exercise**: Write code to light up all LEDs in row 1, then row 2, then row 3 sequentially with 1 second between each.
- **Hint**: Only one row can be active at a time with the column pins driven.
- **Book**: Discovery MB2 - Chapter 6: Display

### ✅ Day 11: LED Matrix Columns
- **Topic**: Controlling columns to select specific LEDs within a row.
- **Exercise**: Light up only the corner LEDs of the matrix (positions 1,1 / 1,5 / 5,1 / 5,5).
- **Hint**: To display an LED at (row, col), set the row high and the column low.
- **Book**: Discovery MB2 - Chapter 6: Display

### ✅ Day 12: Delay Mechanisms
- **Topic**: Busy-wait delays vs timer-based delays and their trade-offs.
- **Exercise**: Implement a busy-wait delay function that counts to a large number. Compare its accuracy to the HAL's `Delay` type.
- **Hint**: Busy-wait burns CPU cycles; timers let you do other work.
- **Book**: Discovery MB2 - Chapter 6: Timers

### ✅ Day 13: LED Patterns Part 1
- **Topic**: Creating simple animations by sequencing LED states.
- **Exercise**: Create a "progress bar" animation that fills the top row from left to right, then clears it.
- **Hint**: Store LED states in a 5x5 boolean array for easier manipulation.
- **Book**: Discovery MB2 - Chapter 6: Display Patterns

### ✅ Day 14: LED Patterns Part 2
- **Topic**: Creating more complex patterns using loops and data structures.
- **Exercise**: Create a rotating pattern that moves around the outer edge of the LED matrix.
- **Hint**: Use an array of (row, col) tuples to define the pattern sequence.
- **Book**: Discovery MB2 - Chapter 6: Display Patterns

### ✅ Day 15: Binary Representations
- **Topic**: Using binary literals and bit manipulation to represent LED states compactly.
- **Exercise**: Represent each row of the LED matrix as a 5-bit number. Display the pattern `0b11111`, `0b10001`, `0b10001`, `0b10001`, `0b11111` (a square).
- **Hint**: Each bit represents one LED in the row.
- **Book**: Discovery MB2 - Chapter 6

### ✅ Day 16: Button Input Basics
- **Topic**: Reading digital input from the micro:bit's buttons A and B.
- **Exercise**: Write a program that reads button A and lights the center LED when pressed.
- **Hint**: Configure the button pin as `into_pullup_input()` - the button pulls the pin low when pressed.
- **Book**: Discovery MB2 - Chapter 7: GPIO Input

### ✅ Day 17: Button Polling
- **Topic**: Continuously checking button state in a loop (polling).
- **Exercise**: Light different LEDs based on which button is pressed: A = left LED, B = right LED, A+B = center LED.
- **Hint**: Use `is_low()` or `is_high()` to check the button state.
- **Book**: Discovery MB2 - Chapter 7: Buttons

### ✅ Day 18: Debouncing Buttons
- **Topic**: Understanding switch bounce and implementing software debouncing.
- **Exercise**: Implement a simple debounce by requiring the button to be pressed for 50ms before registering.
- **Hint**: Read the button state, delay, then check again to confirm.
- **Book**: Discovery MB2 - Chapter 7

### ✅ Day 19: LED Counter
- **Topic**: Combining buttons and LEDs to create an interactive counter.
- **Exercise**: Create a binary counter displayed on the bottom row. Button A increments, Button B decrements (0-31).
- **Hint**: Convert the counter value to binary and display each bit as an LED.
- **Book**: Discovery MB2 - Chapter 7

### ✅ Day 20: Mini-Project - LED Game
- **Topic**: Building a simple reaction game combining all learned concepts.
- **Exercise**: Random LED lights up. Player presses button when center LED is lit. Track successes on the display.
- **Hint**: Use a simple PRNG or counter-based random selection for LED position.
- **Book**: Discovery MB2 - Chapters 6-7

---

## Phase 2: Registers & The "Safety" Transition (Days 21-25)

**Goal:** Move immediately from the "hard way" (unsafe pointers) to the "Rust way" (Type Safe API) as described at the end of Chapter 7.

### ✅ Day 21: Memory-Mapped I/O Introduction
- **Topic**: Understanding how peripherals are accessed through memory addresses.
- **Exercise**: Read the Discovery MB2 Chapter 9 introduction. Identify the base address for GPIO Port 0 in the nRF52833 datasheet.
- **Hint**: Peripherals are accessed just like regular memory, but at special addresses.
- **Book**: Discovery MB2 - Chapter 9: Registers

### Day 22: Type Safe Manipulation
*   **Topic**: Using the Type Safe API to manipulate registers without `unsafe` blocks.
*   **Exercise**: Open `examples/type-safe.rs`. Modify the code to turn on the bottom LED row using `p0.out.modify` instead of raw pointers. Notice how you don't need `unsafe` anymore.
*   **Hint**: The `modify` method takes a closure. You can access pins like `w.pin19().set_bit()`.
*   **Book**: Chapter 7 - Registers (Section: "Type safe manipulation", Page 98)

### Day 23: Reading from Registers
*   **Topic**: Reading register states using the safe API.
*   **Exercise**: Run `cargo embed` on your type-safe code. Use GDB to print `*p0`. Observe how the GDB output shows the register block structure rather than just raw memory addresses.
*   **Hint**: The book shows `print *p0` in GDB outputting a `RegisterBlock`. This confirms you are using the safe abstraction.
*   **Book**: Chapter 7 - Registers (Page 100)

### Day 24: Compiler Optimizations (Release Mode)
*   **Topic**: How the compiler optimizes code and why `volatile` operations matter.
*   **Exercise**: Run `examples/volatile.rs` using `cargo objdump` (as shown in the book) to see the assembly code. Compare the debug build vs. the release build.
*   **Hint**: In release mode, the compiler might remove your code if it thinks the code does nothing (like a busy wait loop without volatile read/writes).
*   **Book**: Chapter 7 - Registers (Section: "(mis)Optimization", Page 85-89)

### Day 25: Phase 2 Review & Cleanup
*   **Topic**: Reviewing the `07-registers` directory.
*   **Exercise**: Read the "Spooky action at a distance" section to understand why modifying one bit might affect others if you aren't careful, and how the hardware handles this.
*   **Hint**: `OUTSET` and `OUTCLR` registers allow you to change specific bits without reading the whole register first.
*   **Book**: Chapter 7 - Registers (Page 95)

---

## Phase 3: Serial Communication (UART) (Days 26-35)

**Goal:** enabling the micro:bit to send text to your computer so we can debug with `println!` logic.

### Day 26: Serial Basics & Tooling
*   **Topic**: Understanding UART and setting up a terminal.
*   **Exercise**: Install/configure `minicom` (Linux/Mac) or `PuTTY` (Windows) as described in the book. Verify you can connect to the device.
*   **Hint**: Baud rate is 115200. You won't see anything yet, just get the window open.
*   **Book**: Chapter 8 - Serial Communication (Pages 107-113)

### Day 27: Sending a Single Byte
*   **Topic**: Initializing the UARTE peripheral.
*   **Exercise**: Run `examples/send-byte.rs`. Initialize the UARTE with the correct pins and baud rate. Send the character 'X' to your computer.
*   **Hint**: You need to initialize the `Uarte` struct and wrap it in `serial_setup::UartePort`.
*   **Book**: Chapter 8 - Serial Communication (Page 115)

### Day 28: Sending a String (Naive)
*   **Topic**: Iterating over bytes to send a sentence.
*   **Exercise**: Run `examples/naive-send-string.rs`. Use a `for` loop to iterate over a byte string `b"Hello..."` and send it byte-by-byte.
*   **Hint**: The `write()` function blocks until the byte is sent.
*   **Book**: Chapter 8 - Serial Communication (Page 119)

### Day 29: Using `core::fmt::Write`
*   **Topic**: Using the formatted write macro (`write!`).
*   **Exercise**: Run `examples/send-string.rs`. Replace the manual loop with the `write!` macro. Note how this allows you to format strings easily.
*   **Hint**: This requires `use core::fmt::Write;`.
*   **Book**: Chapter 8 - Serial Communication (Page 120)

### Day 30: Receiving Data
*   **Topic**: Reading input from your keyboard to the micro:bit.
*   **Exercise**: Run `examples/receive-byte.rs`. Type keys in your terminal and watch them appear in the RTT console (using `rprintln!`).
*   **Hint**: The loop uses `serial.read()`. It will wait (block) until you press a key.
*   **Book**: Chapter 8 - Serial Communication (Page 121)

### Day 31: The Echo Server
*   **Topic**: Building a bidirectional communication loop.
*   **Exercise**: Write a program that reads a byte from the computer and immediately sends it back. This makes your terminal act like a typewriter.
*   **Hint**: Inside the loop: `let byte = serial.read(); serial.write(byte);`.
*   **Book**: Chapter 8 - Serial Communication (Page 122)

### Day 32: Reverse String Challenge (Setup)
*   **Topic**: Setting up a buffer for string manipulation.
*   **Exercise**: Initialize a `heapless::Vec` with a capacity of 32 bytes.
*   **Hint**: Embedded systems don't have a heap, so we use `heapless::Vec` which is a fixed-size array that acts like a vector.
*   **Book**: Chapter 8 - Serial Communication (Page 123)

### Day 33: Reverse String Challenge (Logic)
*   **Topic**: Storing input until 'Enter' is pressed.
*   **Exercise**: Modify your loop to push received bytes into the buffer. If the byte is `\r` (Enter), break the loop.
*   **Hint**: Handle the `buffer.push` error case (if the user types too many characters).
*   **Book**: Chapter 8 - Serial Communication (Page 126)

### Day 34: Reverse String Challenge (Output)
*   **Topic**: Processing the buffer and sending it back.
*   **Exercise**: Once 'Enter' is detected, iterate through the buffer in reverse (`.iter().rev()`) and send the characters back.
*   **Hint**: Don't forget to flush the serial port after writing!
*   **Book**: Chapter 8 - Serial Communication (Page 126)

### Day 35: Review & Polish
*   **Topic**: Reviewing the Serial module.
*   **Exercise**: Read the solution in the book carefully. Compare it to your code. Ensure you understand `heapless::Vec`.
*   **Hint**: Serial communication is the primary way we debug complex logic later.
*   **Book**: Chapter 8 - Serial Communication (Page 126)

---

## Phase 4: I2C and Sensors (Days 36-50)

**Goal:** Talk to the Accelerometer and Magnetometer chips using the I2C protocol.

### Day 36: I2C Protocol Basics
*   **Topic**: Understanding Clock (SCL) and Data (SDA) lines.
*   **Exercise**: Read the theory section on I2C. Understand the "Controller -> Target" relationship.
*   **Hint**: I2C allows multiple sensors on the same two wires.
*   **Book**: Chapter 9 - I2C (Pages 127-130)

### Day 37: Reading a Register
*   **Topic**: Reading the "Who Am I" register manually.
*   **Exercise**: Run `examples/chip-id.rs`. Configure the `twim` (Two-Wire Interface Master). Read the `ACCELEROMETER_ID_REG`.
*   **Hint**: You need to write the address you want to read to the bus, then read the response.
*   **Book**: Chapter 9 - I2C (Pages 132-133)

### Day 38: Using the Driver (Accelerometer)
*   **Topic**: Using the `lsm303agr` crate.
*   **Exercise**: Run `examples/show-accel.rs`. Initialize the `Lsm303agr` struct. Read the X, Y, Z acceleration data.
*   **Hint**: Using a driver crate saves you from looking up register numbers in a datasheet manually.
*   **Book**: Chapter 9 - I2C (Pages 135-136)

### Day 39: The "Punch-o-meter" (Concept)
*   **Topic**: Measuring max G-force.
*   **Exercise**: Read the requirements for the Punch-o-meter challenge.
*   **Hint**: You need to detect when acceleration exceeds a threshold (start punch) and track the max value until it drops.
*   **Book**: Chapter 11 - Accelerometer (Page 152)

### Day 40: The "Punch-o-meter" (Implementation)
*   **Topic**: Implementing the logic.
*   **Exercise**: Modify the loop to track `max_g`. Update `max_g` only if the current reading is higher.
*   **Hint**: Use `sensor.set_accel_scale` to allow measuring higher G-forces (up to 16G).
*   **Book**: Chapter 11 - Accelerometer (Page 153)

### Day 41: Magnetometer Basics
*   **Topic**: Reading the compass sensor.
*   **Exercise**: Run `examples/magnitude.rs`. Read the X, Y, Z magnetic field data.
*   **Hint**: The Z-axis points "into the floor" because the chip is on the back of the board.
*   **Book**: Chapter 10 - Magnetometer (Page 140)

### Day 42: Math in `no_std`
*   **Topic**: Calculating magnitude using `libm`.
*   **Exercise**: Calculate the total strength of the magnetic field: `sqrt(x^2 + y^2 + z^2)`.
*   **Hint**: Standard math functions aren't available in `no_std`. We import `sqrtf` from the `libm` crate.
*   **Book**: Chapter 10 - Magnetometer (Page 142)

### Day 43: Calibration (Theory)
*   **Topic**: Why is the compass wrong?
*   **Exercise**: Read about "Hard Iron" offsets. Understand that you need to find the Min and Max values for X and Y by rotating the board.
*   **Hint**: Calibration code is complex; the book provides a library for it, but understanding the *why* is important.
*   **Book**: Chapter 10 - Magnetometer (Page 218 - Appendix)

### Day 44: LED Compass Challenge (Math)
*   **Topic**: Calculating heading with `atan2`.
*   **Exercise**: Use `atan2f(y, x)` to calculate the angle of the board relative to North.
*   **Hint**: The output is in radians (-PI to +PI).
*   **Book**: Chapter 10 - Magnetometer (Page 145)

### Day 45: LED Compass Challenge (Display)
*   **Topic**: Visualizing direction.
*   **Exercise**: Map the calculated angle to one of the outer LEDs on the 5x5 matrix.
*   **Hint**: The book provides an `indices` array that maps directions to LED coordinates.
*   **Book**: Chapter 10 - Magnetometer (Pages 146-149)

---

## Phase 5: Interrupts & Concurrency (Days 46-65)

**Goal:** Stop "polling" (busy waiting) and start letting the hardware wake up the CPU only when needed.

### Day 46: Interrupt Theory
*   **Topic**: How the CPU handles interruptions.
*   **Exercise**: Read the section on the NVIC (Nested Vectored Interrupt Controller) and the Stack.
*   **Hint**: When an interrupt happens, the CPU pauses your main loop, saves its state, runs the handler, and resumes.
*   **Book**: Chapter 12 - Interrupts (Page 155)

### Day 47: Defining an ISR
*   **Topic**: Writing your first Interrupt Service Routine.
*   **Exercise**: Run `examples/poke.rs`. Define a function `fn GPIOTE()` with the `#[interrupt]` attribute.
*   **Hint**: If you don't clear the event in the handler, the interrupt will fire infinitely (looping).
*   **Book**: Chapter 12 - Interrupts (Page 157)

### Day 48: The Panic Problem
*   **Topic**: What happens inside an interrupt.
*   **Exercise**: Experiment with adding a `panic!()` inside the interrupt handler to see how it stops the program.
*   **Hint**: This confirms the interrupt is actually firing.
*   **Book**: Chapter 12 - Interrupts (Page 158)

### Day 49: Sharing Data (Theory)
*   **Topic**: The problem with global variables in Rust.
*   **Exercise**: Read about `static` variables and why `static mut` is unsafe.
*   **Hint**: Interrupts are like threads; they share memory, which can lead to race conditions.
*   **Book**: Chapter 13 - Concurrency (Page 163)

### Day 50: Critical Sections & Mutex
*   **Topic**: Safe data sharing.
*   **Exercise**: Use `critical_section::Mutex` and `RefCell` to share a counter between `main` and the interrupt.
*   **Hint**: You don't "lock" this Mutex. You use `interrupt::free(|cs| ...)` to access the data.
*   **Book**: Chapter 13 - Concurrency (Page 168)

### Day 51: Configuring GPIOTE
*   **Topic**: Button interrupts.
*   **Exercise**: Configure the GPIOTE peripheral to fire an event when Button A is pressed (High to Low transition).
*   **Hint**: You must unmask the interrupt in the NVIC using `unsafe { pac::NVIC::unmask(...) }`.
*   **Book**: Chapter 12 - Interrupts (Page 157)

### Day 52: Sharing Peripherals
*   **Topic**: Moving the GPIOTE peripheral to a global.
*   **Exercise**: Run `examples/count.rs`. Use `LockMut` (a helper type) to move the GPIOTE peripheral into a global static so the interrupt handler can access it to clear events.
*   **Hint**: Peripherals are singletons; you can't create them twice.
*   **Book**: Chapter 13 - Concurrency (Page 170)

### Day 53: Counting Interrupts
*   **Topic**: A button counter.
*   **Exercise**: Increment an `AtomicUsize` counter every time the button interrupt fires. Print the count in the main loop.
*   **Hint**: Using `Atomic` types avoids the need for a full Mutex for simple numbers.
*   **Book**: Chapter 13 - Concurrency (Page 170)

### Day 54: The Bouncing Button
*   **Topic**: Hardware reality.
*   **Exercise**: Observe that one button press might trigger multiple interrupts (bouncing).
*   **Hint**: Mechanical switches are "noisy" electrically.
*   **Book**: Chapter 13 - Concurrency (Page 173)

### Day 55: Debouncing with Timers
*   **Topic**: Using a timer to ignore bounce.
*   **Exercise**: In the interrupt, start a Timer. Ignore subsequent interrupts until the timer expires.
*   **Hint**: This requires sharing both the GPIOTE and the TIMER globally.
*   **Book**: Chapter 13 - Concurrency (Page 174)

---

## Phase 6: Audio & Advanced Control (Days 56-65)

**Goal:** Using Timers and PWM (Pulse Width Modulation) basics to make sound.

### Day 56: The Speaker
*   **Topic**: How the micro:bit speaker works.
*   **Exercise**: Read about the Speaker pin. It is a piezoelectric speaker controlled by a GPIO pin.
*   **Hint**: High = Push out, Low = Pull in.
*   **Book**: Chapter 15 - The MB2 Speaker (Page 177)

### Day 57: Making a Tone (Square Wave)
*   **Topic**: Generating a frequency.
*   **Exercise**: Run `examples/square-wave.rs`. Toggle the speaker pin High/Low with a delay in between.
*   **Hint**: Frequency = 1 / Period. To get 440Hz, you need a specific delay.
*   **Book**: Chapter 15 - The MB2 Speaker (Page 178)

### Day 58: Interrupt-Driven Sound
*   **Topic**: Non-blocking audio.
*   **Exercise**: Move the toggle logic into a Timer interrupt. This allows the main loop to do other things (like waiting) while the sound plays.
*   **Hint**: This is the "Siren" challenge.
*   **Book**: Chapter 15 - The MB2 Speaker (Page 179)

### Day 59: The Siren Logic
*   **Topic**: Changing frequency over time.
*   **Exercise**: In the interrupt handler, change the timer's `cc` (capture compare) value to change the frequency of the next interrupt.
*   **Hint**: This creates a "sweeping" sound.
*   **Book**: Chapter 15 - The MB2 Speaker (Page 180)

### Day 60: The Siren Structure
*   **Topic**: Organizing the siren code.
*   **Exercise**: Implement the `Siren` struct that holds the state (current frequency, time, pin state).
*   **Hint**: The `Siren` struct needs to be shared globally (Mutex) so the interrupt can update it.
*   **Book**: Chapter 15 - The MB2 Speaker (Page 180)

---

## Phase 7: The Snake Game (Days 61-99)

**Goal:** The Final Project. Building a complete game engine using modules, non-blocking display, and game logic.

### Day 61: Modularity
*   **Topic**: Organizing a large project.
*   **Exercise**: Create the file structure: `src/main.rs`, `src/game.rs`, `src/display.rs`, `src/controls.rs`.
*   **Hint**: Read about the `mod` keyword in Rust.
*   **Book**: Chapter 16 - Snake Game (Page 185)

### Day 62: Game Logic - Coordinates
*   **Topic**: Defining the grid.
*   **Exercise**: Create `src/game/coords.rs`. Define the `Coords` struct (row, col) and the logic for checking if a point is out of bounds.
*   **Hint**: The grid is 5x5.
*   **Book**: Chapter 16 - Snake Game (Page 187)

### Day 63: Random Number Generation (RNG)
*   **Topic**: Generating random food positions.
*   **Exercise**: Implement the `Prng` struct in `src/game/rng.rs` using a simple Xorshift algorithm. Seed it using the micro:bit's hardware RNG.
*   **Hint**: Hardware RNG is slow; software PRNG is fast.
*   **Book**: Chapter 16 - Snake Game (Page 188)

### Day 64: The Snake Struct
*   **Topic**: Defining the snake.
*   **Exercise**: Create `src/game/snake.rs`. Use a `Queue` (from `heapless` crate) to store the snake's body segments.
*   **Hint**: `heapless` allows us to have a list of coordinates without using a dynamic memory allocator (heap).
*   **Book**: Chapter 16 - Snake Game (Page 190)

### Day 65: Movement Logic
*   **Topic**: Moving the snake.
*   **Exercise**: Implement `step()` logic. Add the new head position to the queue and remove the tail (unless eating food).
*   **Hint**: Handle wrapping around the screen edges here.
*   **Book**: Chapter 16 - Snake Game (Page 193)

### Day 66: Game State
*   **Topic**: Win/Loss conditions.
*   **Exercise**: Implement `GameStatus` enum (Won, Lost, Ongoing). Check for self-collision or full grid.
*   **Hint**: If the new head coordinate is already in the snake's body set, it's a collision.
*   **Book**: Chapter 16 - Snake Game (Page 189)

### Day 67: Controls Module - Initialization
*   **Topic**: Setting up buttons for the game.
*   **Exercise**: In `src/controls.rs`, initialize the GPIOTE channels for Button A and B.
*   **Hint**: Reuse the interrupt logic we learned in Phase 5.
*   **Book**: Chapter 16 - Snake Game (Page 196)

### Day 68: Controls Module - Interrupts
*   **Topic**: Handling game input.
*   **Exercise**: Write the interrupt handler to update a global `Turn` enum when buttons are pressed.
*   **Hint**: Button A = Left, Button B = Right.
*   **Book**: Chapter 16 - Snake Game (Page 198)

### Day 69: Display Module - Non-blocking
*   **Topic**: Why non-blocking?
*   **Exercise**: Read about the non-blocking display driver. It allows variable brightness (greyscale) which helps distinguish the head from the tail.
*   **Hint**: It uses a Timer interrupt to flash LEDs very fast.
*   **Book**: Chapter 16 - Snake Game (Page 200)

### Day 70: Display Initialization
*   **Topic**: Setting up the display timer.
*   **Exercise**: In `src/display.rs`, initialize `microbit::display::nonblocking::Display`.
*   **Hint**: You need to enable the TIMER1 interrupt.
*   **Book**: Chapter 16 - Snake Game (Page 201)

### Day 71: Display Interrupt
*   **Topic**: Driving the LEDs.
*   **Exercise**: Implement the `TIMER1` interrupt handler. It simply calls `display.handle_display_event()`.
*   **Hint**: This must be extremely fast to prevent flickering.
*   **Book**: Chapter 16 - Snake Game (Page 202)

### Day 72: Rendering the Game
*   **Topic**: Converting game state to pixels.
*   **Exercise**: Implement `game_matrix()` in `src/game.rs`. It converts the snake coordinates into a 5x5 array of brightness values.
*   **Hint**: Head = Bright, Tail = Dim, Food = Medium.
*   **Book**: Chapter 16 - Snake Game (Page 195)

### Day 73: Final Assembly - The Main Loop
*   **Topic**: Putting it all together.
*   **Exercise**: In `main.rs`, initialize all modules. Create the game loop: `game.step()`, render image, sleep.
*   **Hint**: Use `timer.delay_ms()` to control the game speed.
*   **Book**: Chapter 16 - Snake Game (Page 204)

### Day 74: Tuning the Game
*   **Topic**: Polish.
*   **Exercise**: Adjust the speed. Make it get faster as you eat more food.
*   **Hint**: `step_len_ms` calculation on Page 194.
*   **Book**: Chapter 16 - Snake Game (Page 194)

### Day 75: Debugging the Game
*   **Topic**: Using RTT for game logic.
*   **Exercise**: Add `rprintln!` statements to print the score and current state. Verify the logic works even if the display looks weird.
*   **Book**: Chapter 16 - Snake Game

### Days 76: Deep Dive Review & Extension
*   **Topic**: Re-reading and consolidating.
*   **Exercise**: The book suggests "What's left for you to explore" on Page 206.
*   **Task**: Pick one topic (DMA, SPI, or Async) and read the summary.
*   **Book**: Chapter 17 - Next Steps (Page 206)

---

## Phase 8: The Analog World (ADC & Temp) (Days 77-81)

**Goal:** The book skipped Analog-to-Digital Converters (Page 207). We will learn to read "real world" values, not just 0s and 1s.

### Day 77: ADC Theory & The SAADC
*   **Topic**: Analog-to-Digital Conversion on the nRF52833.
*   **Exercise**: Read the documentation for `Saadc` (Successive Approximation ADC) in the `nrf52833-hal` crate. The micro:bit v2 edge connector pins (0, 1, 2) can be used as analog inputs.
*   **Hint**: Digital signals are square (0V or 3V). Analog signals are curves (0.5V, 1.2V, etc.). The ADC turns voltage into a number (0 to 16383).
*   **Resource**: [nrf52833-hal Saadc Docs](https://docs.rs/nrf52833-hal/latest/nrf52833_hal/saadc/index.html)

### Day 78: Reading the Internal Temperature
*   **Topic**: Using the internal Temp sensor.
*   **Exercise**: The nRF52 chip has a built-in thermometer. Initialize the `Temp` struct from the HAL and read the value. Print it via RTT (`rprintln!`).
*   **Hint**: `let mut temp = Temp::new(board.TEMP); let val = temp.measure();`.
*   **Resource**: [microbit::hal::Temp Docs](https://docs.rs/nrf52833-hal/latest/nrf52833_hal/temp/struct.Temp.html)

### Day 79: The Microphone (ADC in Practice)
*   **Topic**: The Microphone is an analog sensor.
*   **Exercise**: The micro:bit v2 microphone is connected to the ADC. Initialize the microphone pin (`board.microphone`) and read the raw analog value in a loop. Shout at the board and watch the numbers change.
*   **Hint**: You need to enable the microphone bias (power it up) using the `microphone_pin`.
*   **Resource**: [microbit-v2 Board Struct](https://docs.rs/microbit-v2/latest/microbit/struct.Board.html)

### Day 80: Visualizing Sound
*   **Topic**: Mapping Analog values to the LED Matrix.
*   **Exercise**: Create a "VU Meter". If the microphone value is low, light up row 5. If it's loud, light up rows 5, 4, 3, 2, 1.
*   **Hint**: You will need to determine a "baseline" (silence) value and subtract it from your reading to measure loudness.
*   **Resource**: Your previous LED Matrix code.

### Day 81: Light Sensing (The LED Trick)
*   **Topic**: Using LEDs as sensors.
*   **Exercise**: The LED matrix can act as a light sensor (reversed bias). The `microbit-v2` crate abstracts this. Use `display.read_light_level()` (or similar in the BSP) to dim the display when the room is dark.
*   **Hint**: LEDs generate a tiny current when light hits them.
*   **Book**: Discovery PDF - Page 207 (Concept mentioned)

---

## Phase 9: Advanced Communication (SPI & DMA) (Days 82-87)

**Goal:** The book mentions SPI (Page 208) and DMA (Page 206). We will implement a "Loopback" test to learn these.

### Day 82: SPI Theory vs I2C
*   **Topic**: Serial Peripheral Interface.
*   **Exercise**: Understand the difference: SPI uses 4 wires (SCK, MOSI, MISO, CS) and is faster/simpler than I2C.
*   **Hint**: We don't have an SPI screen, so we will connect the micro:bit to *itself*.
*   **Resource**: [Embedded Rust Book - SPI](https://docs.rust-embedded.org/book/peripherals/spi.html)

### Day 83: Wiring a Loopback
*   **Topic**: Physical setup.
*   **Exercise**: Use a jumper wire (or alligator clip) to connect **Pin 15 (MOSI)** to **Pin 14 (MISO)** on the micro:bit edge connector.
*   **Hint**: MOSI = Master Out Slave In. MISO = Master In Slave Out. If we connect them, whatever we send, we should immediately receive back.
*   **Resource**: [micro:bit v2 Pinout](https://tech.microbit.org/hardware/edge-connector/)

### Day 84: Configuring SPI
*   **Topic**: Initializing the `Spim` peripheral.
*   **Exercise**: Initialize `nrf52833_hal::Spim`. Use `board.spi_pins` (or manually configure P0.13 as SCK, P0.15 as MOSI, P0.14 as MISO).
*   **Hint**: The nRF52 uses `Spim` (SPI Master with EasyDMA).
*   **Resource**: [nrf52833-hal Spim Docs](https://docs.rs/nrf52833-hal/latest/nrf52833_hal/spim/index.html)

### Day 85: The Transfer (DMA in action)
*   **Topic**: Sending and Receiving simultaneously.
*   **Exercise**: Create a buffer `let tx_buf = [1, 2, 3, 4];` and an empty `rx_buf`. Use `spim.transfer(&mut rx_buf, &tx_buf)`. Verify `rx_buf` equals `[1, 2, 3, 4]`.
*   **Hint**: Remove the wire and run it again. It should fail (receive zeros). This proves DMA moved the data.
*   **Book**: Discovery PDF - Page 206 (DMA Concept)

### Day 86: UART with DMA
*   **Topic**: Sending huge strings efficiently.
*   **Exercise**: Look at your Serial code from Day 30. The `Uarte` struct uses DMA automatically. Create a 255-byte string and send it.
*   **Hint**: The CPU doesn't copy byte-by-byte. It gives the RAM address to the UARTE peripheral and says "Go".
*   **Resource**: [nrf52833-hal Uarte Docs](https://docs.rs/nrf52833-hal/latest/nrf52833_hal/uarte/index.html)

### Day 87: Blocking vs Non-Blocking Transfer
*   **Topic**: Understanding `transfer` vs `transfer_split`.
*   **Exercise**: Read the docs for `transfer_split`. This allows you to start a transfer and get a `future` or `token` back, doing other work while the data moves.
*   **Hint**: This is the precursor to Async Rust.
*   **Resource**: [embedded-hal SPI Traits](https://docs.rs/embedded-hal/latest/embedded_hal/spi/index.html)

---

## Phase 10: RTIC - Professional Concurrency (Days 88-93)

**Goal:** The PDF explicitly recommends RTIC (Page 211) as the solution to the messy interrupt handling we did in the Snake game.

### Day 88: RTIC Setup
*   **Topic**: What is RTIC?
*   **Exercise**: Create a new project folder. Add `cortex-m-rtic` to dependencies. Read the RTIC "By Example" Introduction.
*   **Hint**: RTIC uses a macro `#[app]` to manage shared resources safely, replacing our manual `CriticalSection` and `Mutex` code.
*   **Resource**: [RTIC Book](https://rtic.rs/2/book/en/)

### Day 89: RTIC Blinky
*   **Topic**: Your first RTIC task.
*   **Exercise**: Port the "Timer Blinky" (Day 52) to RTIC. Create an `init` task that sets up the timer and an `idle` task that loops.
*   **Hint**: RTIC handles the `entry` point for you.
*   **Resource**: [RTIC Book - Simple Example](https://rtic.rs/2/book/en/by-example/app.html)

### Day 90: Hardware Tasks (Interrupts)
*   **Topic**: Handling buttons in RTIC.
*   **Exercise**: Define a task `#[task(binds = GPIOTE, ...)]`. Move your button handling logic here. Notice you don't need `unsafe` to unmask interrupts; RTIC does it.
*   **Hint**: RTIC automates the NVIC configuration we did manually on Day 51.
*   **Resource**: [RTIC Book - Hardware Tasks](https://rtic.rs/2/book/en/by-example/tasks/hardware_tasks.html)

### Day 91: Scheduled Software Tasks
*   **Topic**: The "Siren" logic improved.
*   **Exercise**: Instead of a hardware timer interrupt, use RTIC's software tasks. Create a task `blink_led`. Schedule it to run in 1 second. When it runs, toggle LED and schedule itself again.
*   **Hint**: `cx.schedule.blink_led(monotonics::now() + 1.secs()).unwrap();`
*   **Resource**: [RTIC Book - Scheduling](https://rtic.rs/2/book/en/by-example/tasks/software_tasks.html)

### Day 92: Shared Resources
*   **Topic**: Fixing the "Global Mutable" problem.
*   **Exercise**: Define a resource `struct Resources { state: bool }`. Access it in `init` and your `button` task using `cx.resources.state.lock(|s| ...)`.
*   **Hint**: This replaces the `static MUTEX<RefCell<...>>` boilerplate from the PDF.
*   **Resource**: [RTIC Book - Resources](https://rtic.rs/2/book/en/by-example/resources.html)

### Day 93: Refactoring Snake (Mental Draft)
*   **Topic**: Architecture Review.
*   **Exercise**: Sketch out on paper how the Snake Game would look in RTIC.
    *   `init`: Setup Board.
    *   `task display_refresh` (High Priority): Multiplexing the LEDs.
    *   `task game_tick` (Lower Priority): Moving the snake.
    *   `task button_press` (Hardware): Changing direction.
*   **Hint**: RTIC solves the "glitchy display" issues by allowing high-priority display refreshing to interrupt low-priority game logic.

---

## Phase 11: The Cutting Edge (Async/Embassy) (Days 94-99)

**Goal:** The PDF mentions **Embassy** on Page 210. This is the modern standard for embedded Rust, making code look like standard synchronous code while being non-blocking.

### Day 94: Async Concepts
*   **Topic**: `async` and `.await` in Embedded.
*   **Exercise**: Read the Embassy "Executor" documentation. Understand that we don't use OS threads; we use a single-core cooperative scheduler.
*   **Hint**: Async allows us to write `timer.after(Duration::from_secs(1)).await` instead of blocking the whole CPU.
*   **Resource**: [Embassy Documentation](https://embassy.dev/book/dev/runtime.html)

### Day 95: Embassy Setup
*   **Topic**: Setting up the nRF runtime.
*   **Exercise**: Create a new project. Add `embassy-nrf`, `embassy-executor`, and `embassy-time`. Configure `main` to be `async fn main`.
*   **Hint**: You will need to enable features for `nrf52833`.
*   **Resource**: [Embassy nRF Examples](https://github.com/embassy-rs/embassy/tree/main/examples/nrf52833)

### Day 96: Async Blinky
*   **Topic**: The Hello World of Async.
*   **Exercise**: Write a loop: `led.set_high(); Timer::after_millis(500).await; led.set_low(); Timer::after_millis(500).await;`.
*   **Hint**: Notice how clean this looks compared to the interrupt state machines!

### Day 97: Async UART (DMA made easy)
*   **Topic**: Zero-cost Async Serial.
*   **Exercise**: Configure the UART in Embassy. Write `uart.write(b"Hello").await`.
*   **Hint**: Embassy uses DMA under the hood. While it "awaits", the CPU can go to sleep (WFI) automatically to save power.

### Day 98: Multiple Tasks (Spawner)
*   **Topic**: Running two things at once.
*   **Exercise**: Spawn two tasks: `task_blink_led` and `task_print_uart`. Give them different delay times. Watch them run "concurrently".
*   **Hint**: `spawner.spawn(task_blink_led()).unwrap();`

### Day 99: Graduation & "Awesome Embedded Rust"
*   **Topic**: The Ecosystem.
*   **Exercise**: Browse the [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust) list. Look for drivers for hardware you might have lying around (LED strips, displays, motors).
*   **Task**: Plan your next project.
*   **Book**: Discovery PDF - Page 210 ("So where to next?")