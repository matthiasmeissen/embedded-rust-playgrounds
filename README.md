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

### Day 3: Your First Code Change
- **Topic**: Modifying the LED blink pattern and understanding basic GPIO output.
- **Exercise**: Change the blink pattern to: on for 1 second, off for 1 second, on for 200ms, off for 200ms (repeating). Experiment with different patterns.
- **Hint**: Look for `board.display_pins` and the `.set_high()` / `.set_low()` methods. The delay is controlled by `timer.delay_ms()`.
- **Book**: Discovery MB2 - Chapter 5

### Day 4: Meet Your Hardware
- **Topic**: Understanding the BBC micro:bit v2 components and capabilities.
- **Exercise**: With your micro:bit in hand, physically identify: the nRF52833 chip (the large black square), the 5x5 LED matrix, buttons A and B, the USB connector, and the edge connector pins. Read Chapter 4 to understand what each does.
- **Hint**: The nRF52833 is an ARM Cortex-M4F processor (the 'F' means floating-point support) with 512KB Flash and 128KB RAM.
- **Book**: Discovery MB2 - Chapter 4: Meet Your Hardware

### Day 5: Project Structure Deep Dive
- **Topic**: Understanding the anatomy of an embedded Rust project - Cargo.toml, memory.x, and Embed.toml.
- **Exercise**: Examine your project's `Cargo.toml` - identify the key dependencies: `cortex-m`, `cortex-m-rt` (runtime), `nrf52833-hal` (hardware abstraction layer), and `panic-halt`. Read what each one provides.
- **Hint**: `cortex-m-rt` provides the startup code and interrupt vector table. `panic-halt` defines what happens when your program panics (it just halts).
- **Book**: Discovery MB2 - Chapter 5 | Embedded Rust Book - Chapter 5

---

## Phase 1: Basics & LED Control (Days 6-20)

### Day 6: Understanding `no_std`
- **Topic**: Why embedded systems don't use the standard library and what `core` provides.
- **Exercise**: Create a minimal `no_std` program. Try using a `Vec` and observe the compiler error. Replace it with a fixed-size array.
- **Hint**: You can't allocate memory dynamically without `std`!
- **Book**: Embedded Rust Book - Chapter 4: A `no_std` Application

### Day 7: Memory Layout Basics
- **Topic**: Understanding Flash (program storage) vs RAM (runtime data) and the `.text`, `.data`, `.bss` sections.
- **Exercise**: Use `cargo size` to inspect your binary's memory sections. Note which section is largest.
- **Hint**: Flash memory is non-volatile; RAM is volatile but faster.
- **Book**: Embedded Rust Book - Chapter 5: Memory Layout

### Day 8: Hello, LED!
- **Topic**: Controlling a single LED on the micro:bit's LED matrix using GPIO.
- **Exercise**: Light up the center LED (row 3, column 3) of the 5x5 matrix. Keep it on for 2 seconds, then turn it off.
- **Hint**: The LED matrix is multiplexed - you control rows and columns separately.
- **Book**: Discovery MB2 - Chapter 6: Hello World

### Day 9: GPIO Pin Configuration
- **Topic**: Understanding GPIO modes: input, output, push-pull, open-drain.
- **Exercise**: Configure pin P0.21 (LED row 1) as an output. Toggle it on and off 10 times with 500ms delays.
- **Hint**: Use `into_push_pull_output()` to configure a pin for driving LEDs.
- **Book**: Discovery MB2 - Chapter 7: GPIO

### Day 10: LED Matrix Rows
- **Topic**: Understanding row scanning in the LED matrix and how multiplexing works.
- **Exercise**: Write code to light up all LEDs in row 1, then row 2, then row 3 sequentially with 1 second between each.
- **Hint**: Only one row can be active at a time with the column pins driven.
- **Book**: Discovery MB2 - Chapter 6: Display

### Day 11: LED Matrix Columns
- **Topic**: Controlling columns to select specific LEDs within a row.
- **Exercise**: Light up only the corner LEDs of the matrix (positions 1,1 / 1,5 / 5,1 / 5,5).
- **Hint**: To display an LED at (row, col), set the row high and the column low.
- **Book**: Discovery MB2 - Chapter 6: Display

### Day 12: Delay Mechanisms
- **Topic**: Busy-wait delays vs timer-based delays and their trade-offs.
- **Exercise**: Implement a busy-wait delay function that counts to a large number. Compare its accuracy to the HAL's `Delay` type.
- **Hint**: Busy-wait burns CPU cycles; timers let you do other work.
- **Book**: Discovery MB2 - Chapter 6: Timers

### Day 13: LED Patterns Part 1
- **Topic**: Creating simple animations by sequencing LED states.
- **Exercise**: Create a "progress bar" animation that fills the top row from left to right, then clears it.
- **Hint**: Store LED states in a 5x5 boolean array for easier manipulation.
- **Book**: Discovery MB2 - Chapter 6: Display Patterns

### Day 14: LED Patterns Part 2
- **Topic**: Creating more complex patterns using loops and data structures.
- **Exercise**: Create a rotating pattern that moves around the outer edge of the LED matrix.
- **Hint**: Use an array of (row, col) tuples to define the pattern sequence.
- **Book**: Discovery MB2 - Chapter 6: Display Patterns

### Day 15: Binary Representations
- **Topic**: Using binary literals and bit manipulation to represent LED states compactly.
- **Exercise**: Represent each row of the LED matrix as a 5-bit number. Display the pattern `0b11111`, `0b10001`, `0b10001`, `0b10001`, `0b11111` (a square).
- **Hint**: Each bit represents one LED in the row.
- **Book**: Discovery MB2 - Chapter 6

### Day 16: Button Input Basics
- **Topic**: Reading digital input from the micro:bit's buttons A and B.
- **Exercise**: Write a program that reads button A and lights the center LED when pressed.
- **Hint**: Configure the button pin as `into_pullup_input()` - the button pulls the pin low when pressed.
- **Book**: Discovery MB2 - Chapter 7: GPIO Input

### Day 17: Button Polling
- **Topic**: Continuously checking button state in a loop (polling).
- **Exercise**: Light different LEDs based on which button is pressed: A = left LED, B = right LED, A+B = center LED.
- **Hint**: Use `is_low()` or `is_high()` to check the button state.
- **Book**: Discovery MB2 - Chapter 7: Buttons

### Day 18: Debouncing Buttons
- **Topic**: Understanding switch bounce and implementing software debouncing.
- **Exercise**: Implement a simple debounce by requiring the button to be pressed for 50ms before registering.
- **Hint**: Read the button state, delay, then check again to confirm.
- **Book**: Discovery MB2 - Chapter 7

### Day 19: LED Counter
- **Topic**: Combining buttons and LEDs to create an interactive counter.
- **Exercise**: Create a binary counter displayed on the bottom row. Button A increments, Button B decrements (0-31).
- **Hint**: Convert the counter value to binary and display each bit as an LED.
- **Book**: Discovery MB2 - Chapter 7

### Day 20: Mini-Project - LED Game
- **Topic**: Building a simple reaction game combining all learned concepts.
- **Exercise**: Random LED lights up. Player presses button when center LED is lit. Track successes on the display.
- **Hint**: Use a simple PRNG or counter-based random selection for LED position.
- **Book**: Discovery MB2 - Chapters 6-7

---

## Phase 2: Deep Dive - Registers & Memory (Days 21-30)

### Day 21: Memory-Mapped I/O Introduction
- **Topic**: Understanding how peripherals are accessed through memory addresses.
- **Exercise**: Read the Discovery MB2 Chapter 9 introduction. Identify the base address for GPIO Port 0 in the nRF52833 datasheet.
- **Hint**: Peripherals are accessed just like regular memory, but at special addresses.
- **Book**: Discovery MB2 - Chapter 9: Registers

### Day 22: Raw Pointer Access
- **Topic**: Using unsafe Rust to directly manipulate peripheral registers.
- **Exercise**: Turn on an LED by directly writing to the P0.OUT register at address `0x50000504`.
- **Hint**: You'll need `unsafe` blocks and pointer dereferencing: `*(0x50000504 as *mut u32)`.
- **Book**: Discovery MB2 - Chapter 9: Registers

### Day 23: Register Bit Fields
- **Topic**: Understanding how registers are divided into bit fields with specific functions.
- **Exercise**: Read the P0.OUT register documentation. Identify which bit controls P0.21 (LED row 1).
- **Hint**: Bit 21 controls pin P0.21. Use bit masking: `|= 1 << 21` to set it.
- **Book**: Discovery MB2 - Chapter 9: RTRM

### Day 24: Read-Modify-Write Operations
- **Topic**: Safely updating specific bits in a register without affecting others.
- **Exercise**: Toggle a single LED by reading P0.OUT, flipping bit 21 with XOR, and writing it back.
- **Hint**: Use `^=` for XOR toggle: `reg ^= 1 << 21`.
- **Book**: Discovery MB2 - Chapter 9

### Day 25: GPIO Direction Registers
- **Topic**: Configuring pins as inputs or outputs using the DIR register.
- **Exercise**: Manually configure P0.21 as an output by setting bit 21 in the P0.DIR register (address `0x50000514`).
- **Hint**: Setting a bit to 1 in DIR makes it an output; 0 makes it an input.
- **Book**: Discovery MB2 - Chapter 9

### Day 26: Reading Input Registers
- **Topic**: Reading button states from the P0.IN register.
- **Exercise**: Poll the P0.IN register to detect button A presses. Button A is typically on P0.14.
- **Hint**: Check if bit 14 is 0 (button pressed) or 1 (button released).
- **Book**: Discovery MB2 - Chapter 9

### Day 27: Peripheral Access Crates (PAC)
- **Topic**: Using the `nrf52833-pac` crate for type-safe register access.
- **Exercise**: Rewrite your LED control code using the PAC instead of raw pointers. Compare the syntax.
- **Hint**: The PAC provides structured access like `peripherals.P0.out.write(|w| ...)`.
- **Book**: Embedded Rust Book - Chapter 6: Peripherals

### Day 28: Hardware Abstraction Layers (HAL)
- **Topic**: Understanding the HAL's role in providing portable, safe abstractions.
- **Exercise**: Compare raw register code, PAC code, and HAL code for the same LED operation. Note the differences in verbosity and safety.
- **Hint**: HAL is the most portable and safe; raw registers are most direct but dangerous.
- **Book**: Embedded Rust Book - Chapter 6: HAL

### Day 29: Volatile Access
- **Topic**: Why peripheral registers must be accessed with `volatile` reads/writes.
- **Exercise**: Modify a register without `volatile` (plain pointer read/write). Observe how the compiler may optimize away your code.
- **Hint**: Hardware can change register values independently of your code!
- **Book**: Embedded Rust Book - Chapter 5

### Day 30: Review & Mini-Project
- **Topic**: Consolidating register knowledge with a practical application.
- **Exercise**: Implement a "Simon Says" game using only register-level code (no HAL). Show pattern, player repeats.
- **Hint**: Store button press sequences in a fixed-size array.
- **Book**: Discovery MB2 - Chapter 9

---

## Phase 3: Timers & Interrupts (Days 31-45)

### Day 31: Timer Basics
**Topic**: Understanding hardware timers and their capabilities (counting, compare, capture).
**Exercise**: Read the nRF52833 TIMER documentation. Identify how many timers are available and their bit width.
**Hint**: The nRF52833 has 5 timers (TIMER0-TIMER4), each 32-bit.
**Book**: Discovery MB2 - Chapter 6: Timers | Embedded Rust Book - Chapter 11

### Day 32: Configuring a Timer
**Topic**: Setting up a timer to count at a specific frequency.
**Exercise**: Configure TIMER0 to count at 1 MHz (1 microsecond per tick). Print the timer value after 1 second.
**Hint**: Use the prescaler to divide the 16MHz system clock down to 1MHz.
**Book**: Discovery MB2 - Chapter 6: Timers

### Day 33: Timer Delays
**Topic**: Creating accurate delays using timer compare events.
**Exercise**: Replace your busy-wait delays with timer-based delays. Measure the accuracy improvement.
**Hint**: Set a compare value, wait for the compare event flag to be set.
**Book**: Discovery MB2 - Chapter 6: Timers

### Day 34: Millisecond Clock
**Topic**: Using a timer to maintain a system time in milliseconds.
**Exercise**: Create a `millis()` function that returns elapsed milliseconds since startup using TIMER0.
**Hint**: Configure the timer to overflow/reset every millisecond and count overflows.
**Book**: Discovery MB2 - Chapter 6

### Day 35: Multiple Timer Channels
**Topic**: Using multiple compare channels for different timing events.
**Exercise**: Blink two LEDs at different rates (500ms and 250ms) using two compare channels on one timer.
**Hint**: Each channel can have its own compare value and event flag.
**Book**: Discovery MB2 - Chapter 6

### Day 36: Introduction to Interrupts
**Topic**: Understanding interrupt-driven programming vs polling.
**Exercise**: Read about the NVIC (Nested Vectored Interrupt Controller) in the Cortex-M4 documentation.
**Hint**: Interrupts let the CPU do other work and respond only when an event occurs.
**Book**: Embedded Rust Book - Chapter 12: Interrupts

### Day 37: Timer Interrupts
**Topic**: Triggering an interrupt when a timer compare event occurs.
**Exercise**: Configure TIMER0 to generate an interrupt every 1 second. Toggle an LED in the interrupt handler.
**Hint**: Enable the timer interrupt in the NVIC and implement the `TIMER0()` interrupt handler function.
**Book**: Discovery MB2 - Chapter 14: Interrupts

### Day 38: Critical Sections
**Topic**: Protecting shared data between main code and interrupt handlers.
**Exercise**: Create a counter shared between main and an interrupt. Use a critical section to safely read it in main.
**Hint**: Use `cortex_m::interrupt::free(|cs| ...)` to create a critical section.
**Book**: Embedded Rust Book - Chapter 12: Concurrency

### Day 39: Interrupt Priority
**Topic**: Setting interrupt priorities to control which interrupts can preempt others.
**Exercise**: Set up two interrupts with different priorities. Verify that the higher priority one can preempt the lower.
**Hint**: Lower priority numbers = higher priority (0 is highest).
**Book**: Embedded Rust Book - Chapter 12

### Day 40: Button Interrupts (GPIO Interrupts)
**Topic**: Using GPIOTE (GPIO Tasks and Events) to trigger interrupts on button presses.
**Exercise**: Configure button A to generate an interrupt on press. Count button presses in the handler.
**Hint**: Use the GPIOTE peripheral with PORT events or IN channels.
**Book**: Discovery MB2 - Chapter 14

### Day 41: Debouncing with Interrupts
**Topic**: Implementing software debouncing in an interrupt-driven system.
**Exercise**: Modify your button interrupt to ignore bounces by disabling the interrupt temporarily after each press.
**Hint**: Start a timer in the button interrupt and re-enable the button interrupt when the timer expires.
**Book**: Discovery MB2 - Chapter 14

### Day 42: Non-Blocking LED Animations
**Topic**: Using timer interrupts to drive LED animations without blocking code.
**Exercise**: Implement a scrolling pattern that updates in an interrupt while main() counts button presses.
**Hint**: Store the animation state in a static variable updated by the interrupt.
**Book**: Discovery MB2 - Chapter 14

### Day 43: WFI (Wait For Interrupt)
**Topic**: Using the `wfi` instruction to put the CPU to sleep until an interrupt occurs.
**Exercise**: Modify your program to call `wfi()` in the main loop. Measure power consumption difference (conceptually).
**Hint**: `wfi` saves power - the CPU stops until woken by an interrupt.
**Book**: Embedded Rust Book - Chapter 12

### Day 44: Atomic Operations
**Topic**: Using atomic types for lock-free communication between interrupts and main code.
**Exercise**: Replace your critical-section-protected counter with an `AtomicU32`. Compare the code.
**Hint**: Use `.load(Ordering::Relaxed)` and `.store(val, Ordering::Relaxed)`.
**Book**: Embedded Rust Book - Chapter 12: Concurrency

### Day 45: Mini-Project - Stopwatch
**Topic**: Building an interactive stopwatch using interrupts and timers.
**Exercise**: Button A starts/stops. Button B resets. Display elapsed seconds as a binary number on LEDs. Update via interrupt.
**Hint**: Use a timer interrupt at 10Hz to update a deciseconds counter.
**Book**: Discovery MB2 - Chapters 6, 14

---

## Phase 4: Serial Communication (Days 46-55)

### Day 46: UART/Serial Basics
**Topic**: Understanding asynchronous serial communication: baud rate, start/stop bits, parity.
**Exercise**: Read about the UARTE peripheral in the nRF52833 documentation. Note that it uses DMA.
**Hint**: UART = Universal Asynchronous Receiver/Transmitter. Baud rate must match on both sides!
**Book**: Discovery MB2 - Chapter 10: Serial Communication

### Day 47: Serial Configuration
**Topic**: Setting up the UART peripheral with appropriate baud rate and pins.
**Exercise**: Configure UARTE0 for 115200 baud using the micro:bit's USB serial pins. Test by connecting to a terminal.
**Hint**: The micro:bit v2 has a built-in USB-to-serial converter. No external hardware needed!
**Book**: Discovery MB2 - Chapter 10

### Day 48: Sending Data
**Topic**: Transmitting bytes and strings over serial.
**Exercise**: Send "Hello, Embedded Rust!\n" over serial when button A is pressed. Observe in a terminal emulator.
**Hint**: Use `write_all()` or similar methods from the HAL's serial traits.
**Book**: Discovery MB2 - Chapter 10

### Day 49: Formatted Output
**Topic**: Using `core::fmt::Write` to send formatted text over serial.
**Exercise**: Implement the `Write` trait for your serial port. Use `write!()` macro to send formatted strings.
**Hint**: This enables `write!(serial, "Counter: {}", count)` syntax!
**Book**: Discovery MB2 - Chapter 10: `core::fmt`

### Day 50: Receiving Data
**Topic**: Reading bytes from serial and handling input.
**Exercise**: Echo received characters back to the sender. Display 'A' on LEDs when 'A' is received.
**Hint**: Use `read()` to receive one byte at a time. Check for errors!
**Book**: Discovery MB2 - Chapter 10

### Day 51: Line Buffering
**Topic**: Accumulating received characters into lines for processing.
**Exercise**: Build a line buffer that collects characters until '\n' is received, then process the line.
**Hint**: Use a fixed-size array as a circular buffer. Handle buffer overflow gracefully.
**Book**: Discovery MB2 - Chapter 10

### Day 52: Serial Command Parser
**Topic**: Implementing a simple command interpreter over serial.
**Exercise**: Accept commands like "LED ON", "LED OFF", "COUNT" and respond appropriately.
**Hint**: Use pattern matching on the received string slices.
**Book**: Discovery MB2 - Chapter 10

### Day 53: Logging Framework
**Topic**: Using the `defmt` logging framework for embedded debugging.
**Exercise**: Replace your serial `write!()` calls with `defmt::info!()` and friends. Compare the binary size.
**Hint**: `defmt` moves formatting to the host PC, saving precious flash space!
**Book**: Discovery MB2 - Chapter 5 | defmt documentation

### Day 54: Serial Interrupts
**Topic**: Using interrupts for non-blocking serial I/O.
**Exercise**: Set up RX interrupt to receive characters without polling. Send them back in the interrupt handler.
**Hint**: Be careful with shared state between the interrupt and main code!
**Book**: Discovery MB2 - Chapter 10

### Day 55: Mini-Project - Serial Terminal
**Topic**: Building an interactive menu system over serial.
**Exercise**: Create a menu: 1) Show counter, 2) Reset counter, 3) LED pattern. Handle user input.
**Hint**: Display the menu after each command completes.
**Book**: Discovery MB2 - Chapter 10

---

## Phase 5: I2C and Sensors (Days 56-70)

### Day 56: I2C Protocol Overview
**Topic**: Understanding the I2C bus: master/slave, addresses, ACK/NACK, clock stretching.
**Exercise**: Read about the I2C protocol. Draw a timing diagram for a simple read operation.
**Hint**: I2C uses two wires: SDA (data) and SCL (clock). Multiple devices share the bus!
**Book**: Discovery MB2 - Chapter 12: I2C | Embedded Rust Book - Chapter 13

### Day 57: I2C Sensor Overview
**Topic**: Understanding the LSM303AGR accelerometer/magnetometer on the micro:bit v2.
**Exercise**: Read the LSM303AGR datasheet. Find its I2C addresses (there are two - accel and mag).
**Hint**: The accelerometer is at 0x19, the magnetometer at 0x1E.
**Book**: Discovery MB2 - Chapter 12: Hardware

### Day 58: I2C Configuration
**Topic**: Setting up the TWIM (Two-Wire Interface Master) peripheral.
**Exercise**: Configure TWIM0 with the correct SDA/SCL pins for the micro:bit v2. Run an I2C scan to detect devices.
**Hint**: SDA is typically P0.16, SCL is P0.08 on micro:bit v2.
**Book**: Discovery MB2 - Chapter 12: I2C Setup

### Day 59: I2C Write Operations
**Topic**: Sending configuration commands to an I2C sensor.
**Exercise**: Write to the LSM303AGR's CTRL_REG1_A register to power on the accelerometer.
**Hint**: Register address is 0x20. Write 0x57 to enable all axes at 100Hz.
**Book**: Discovery MB2 - Chapter 12

### Day 60: I2C Read Operations
**Topic**: Reading data from I2C sensor registers.
**Exercise**: Read the WHO_AM_I register (0x0F) from the accelerometer. Verify it returns 0x33.
**Hint**: Write the register address, then read the response byte.
**Book**: Discovery MB2 - Chapter 12

### Day 61: Multi-Byte Reads
**Topic**: Reading sequential registers in a single I2C transaction.
**Exercise**: Read all 6 acceleration data registers (X, Y, Z as 16-bit values) in one operation.
**Hint**: Set the MSB of the register address to enable auto-increment.
**Book**: Discovery MB2 - Chapter 12

### Day 62: Accelerometer Data Processing
**Topic**: Converting raw ADC values to physical units (g-forces).
**Exercise**: Read raw accelerometer data and convert to mg (millig). Display direction on LEDs (tilt detector).
**Hint**: At ±2g scale, sensitivity is ~1mg/LSB. Data is 16-bit signed.
**Book**: Discovery MB2 - Chapter 12: Accelerometer

### Day 63: Using the LSM303AGR Driver
**Topic**: Leveraging the `lsm303agr` crate for simplified sensor access.
**Exercise**: Replace your raw I2C code with the `lsm303agr` driver crate. Compare code complexity.
**Hint**: The driver handles register details and data conversion for you!
**Book**: Discovery MB2 - Chapter 12: Using a Driver

### Day 64: Magnetometer Basics
**Topic**: Understanding magnetic field sensing and compass applications.
**Exercise**: Enable the magnetometer and read raw magnetic field values on X, Y, Z axes.
**Hint**: Magnetometer needs different configuration registers than the accelerometer.
**Book**: Discovery MB2 - Chapter 12: Magnetometer

### Day 65: Compass Calculation
**Topic**: Computing heading angle from magnetometer data.
**Exercise**: Calculate heading using `atan2(y, x)`. Display 8 directions (N, NE, E, SE, S, SW, W, NW) on LEDs.
**Hint**: You'll need `libm` or `micromath` for trig functions in `no_std`.
**Book**: Discovery MB2 - Chapter 12

### Day 66: Sensor Calibration
**Topic**: Understanding sensor bias and scaling errors and how to compensate.
**Exercise**: Implement a simple calibration routine: collect min/max values while rotating the board.
**Hint**: Store calibration offsets and scales in static variables.
**Book**: Discovery MB2 - Chapter 12

### Day 67: Tilt-Compensated Compass
**Topic**: Using accelerometer data to correct compass readings when tilted.
**Exercise**: Implement tilt compensation math to get accurate heading regardless of board orientation.
**Hint**: This requires 3D rotation matrix math - it's complex!
**Book**: Discovery MB2 - Chapter 12

### Day 68: Sensor Fusion Basics
**Topic**: Combining multiple sensors for more robust measurements.
**Exercise**: Implement a simple low-pass filter to smooth noisy accelerometer readings.
**Hint**: `filtered = filtered * 0.9 + new_reading * 0.1` is a basic exponential moving average.
**Book**: Discovery MB2 - Chapter 12

### Day 69: Embedded-HAL Traits
**Topic**: Understanding how the `embedded-hal` traits enable portable drivers.
**Exercise**: Examine the `lsm303agr` driver source. Note how it's generic over I2C trait implementations.
**Hint**: This is why the same driver works on nRF52, STM32, and other platforms!
**Book**: Embedded Rust Book - Chapter 7: Portability

### Day 70: Mini-Project - Spirit Level
**Topic**: Building a digital level/inclinometer using the accelerometer.
**Exercise**: Show tilt angle on the LED matrix. Level position shows center LED. Tilt moves the dot accordingly.
**Hint**: Map acceleration values (-2g to +2g) to LED positions (0 to 4).
**Book**: Discovery MB2 - Chapter 12

---

## Phase 6: Advanced Topics & Concurrency (Days 71-85)

### Day 71: RTIC Framework Introduction
**Topic**: Understanding Real-Time Interrupt-driven Concurrency framework.
**Exercise**: Read the RTIC book introduction. Understand resources, tasks, and priorities.
**Hint**: RTIC provides compile-time guarantees about race conditions!
**Book**: [RTIC Book](https://rtic.rs) - Introduction | Embedded Rust Book - Chapter 14

### Day 72: RTIC - Basic Application
**Topic**: Converting a simple interrupt-driven application to RTIC.
**Exercise**: Port your timer interrupt + button LED toggle app to RTIC. Compare code structure.
**Hint**: RTIC tasks replace raw interrupt handlers. Resources replace static mut variables.
**Book**: RTIC Book - Chapter 2

### Day 73: RTIC - Shared Resources
**Topic**: Safely sharing data between tasks using RTIC resources.
**Exercise**: Share a counter between a timer task (incrementing) and button task (displaying). No `unsafe` needed!
**Hint**: RTIC automatically manages critical sections for resource access.
**Book**: RTIC Book - Chapter 3: Resources

### Day 74: RTIC - Message Passing
**Topic**: Using software tasks and spawn/schedule for message passing.
**Exercise**: Create a software task that processes button events. Hardware button task spawns the software task.
**Hint**: Software tasks run at specified priorities but aren't tied to hardware interrupts.
**Book**: RTIC Book - Chapter 4

### Day 75: PWM Basics
**Topic**: Understanding Pulse Width Modulation for LED brightness and motor control.
**Exercise**: Read the nRF52833 PWM peripheral documentation. Note how many channels are available.
**Hint**: PWM creates an analog-like output using digital pulses. Duty cycle controls brightness.
**Book**: Discovery MB2 - Chapter 8: PWM | Embedded Rust Book - Chapter 15

### Day 76: LED Brightness Control
**Topic**: Using PWM to fade LEDs smoothly.
**Exercise**: Fade a single LED from off to full brightness over 2 seconds. Use PWM, not delays!
**Hint**: Map time to duty cycle: 0% → 100%.
**Book**: Discovery MB2 - Chapter 8

### Day 77: Multi-Channel PWM
**Topic**: Controlling multiple LEDs with different brightness levels simultaneously.
**Exercise**: Create a "breathing" pattern on 3 LEDs with different phases (one peaks while others trough).
**Hint**: Use 3 PWM channels with sine-wave-like duty cycle patterns.
**Book**: Discovery MB2 - Chapter 8

### Day 78: Speaker/Buzzer Control
**Topic**: Using PWM to generate audio tones on the micro:bit v2's speaker.
**Exercise**: Play a simple melody (like "Mary Had a Little Lamb") by varying PWM frequency.
**Hint**: Frequency determines pitch, duty cycle determines volume. Middle C = 262 Hz.
**Book**: Discovery MB2 - Chapter 8

### Day 79: DMA (Direct Memory Access)
**Topic**: Understanding how DMA transfers data without CPU intervention.
**Exercise**: Read about the nRF52833's EasyDMA. Understand how UARTE uses it for background transfers.
**Hint**: DMA lets peripherals read/write RAM directly, freeing the CPU for other work.
**Book**: Embedded Rust Book - Chapter 16: DMA

### Day 80: Power Management Basics
**Topic**: Understanding sleep modes and power consumption optimization.
**Exercise**: Measure (conceptually) current draw with CPU active vs in WFI. Configure low-power mode.
**Hint**: The nRF52833 has multiple sleep modes: System ON, System OFF, etc.
**Book**: Embedded Rust Book - Chapter 17: Power

### Day 81: RTC (Real-Time Counter)
**Topic**: Using the low-power RTC for long-duration timing and sleep.
**Exercise**: Use RTC instead of TIMER for a seconds counter. Compare power consumption (conceptual).
**Hint**: RTC runs at 32.768 kHz and can wake the CPU from deep sleep.
**Book**: Discovery MB2 - Chapter 6

### Day 82: Panic Handlers
**Topic**: Understanding panic behavior in embedded systems and custom panic handlers.
**Exercise**: Implement a custom panic handler that blinks an LED in an SOS pattern before halting.
**Hint**: `#[panic_handler]` function is called when your program panics. No stdlib to catch it!
**Book**: Embedded Rust Book - Chapter 8: Panicking

### Day 83: Error Handling Patterns
**Topic**: Using `Result` types effectively in embedded code without allocating.
**Exercise**: Refactor your I2C code to return `Result<T, Error>`. Handle errors by retrying or displaying on LEDs.
**Hint**: Avoid `.unwrap()` in production code! Always handle errors explicitly.
**Book**: Embedded Rust Book - Chapter 9

### Day 84: Bootloader Basics
**Topic**: Understanding how bootloaders work and enabling firmware updates.
**Exercise**: Read about the nRF52833 bootloader options (Nordic DFU, UF2). Understand memory partitioning.
**Hint**: Bootloader lives in protected flash and can update the main application.
**Book**: Embedded Rust Book - Chapter 18

### Day 85: Mini-Project - Music Player
**Topic**: Combining PWM audio, buttons, and song data structures.
**Exercise**: Store 3 melodies in flash. Buttons select songs. Play using PWM on speaker. Show playing/paused on LEDs.
**Hint**: Store notes as (frequency, duration) tuples in const arrays.
**Book**: Discovery MB2 - Chapters 6, 7, 8

---

## Phase 7: Integration & Final Project (Days 86-99)

### Day 86: Project Planning
**Topic**: Designing a complete embedded application with requirements and architecture.
**Exercise**: Plan your final project. Write requirements, draw a block diagram, list needed peripherals.
**Hint**: Keep scope manageable! A working simple project beats an incomplete complex one.
**Book**: Discovery MB2 - Chapter 13

### Day 87: Snake Game - Part 1 (Setup)
**Topic**: Initializing game state and display structures.
**Exercise**: Create data structures for snake position (array of coordinates), food position, and direction.
**Hint**: Use a circular buffer for the snake body. Max length = 25 (full matrix).
**Book**: Discovery MB2 - Chapter 13: Snake Game

### Day 88: Snake Game - Part 2 (Movement)
**Topic**: Implementing game logic for snake movement and collision.
**Exercise**: Move the snake head based on direction. Detect wall collisions and self-collisions.
**Hint**: Check if new head position equals any body segment position for self-collision.
**Book**: Discovery MB2 - Chapter 13

### Day 89: Snake Game - Part 3 (Input)
**Topic**: Handling button input to change snake direction.
**Exercise**: Button A turns left, Button B turns right (relative to current direction). Prevent 180° turns.
**Hint**: Store direction as an enum: North, East, South, West.
**Book**: Discovery MB2 - Chapter 13

### Day 90: Snake Game - Part 4 (Food & Growth)
**Topic**: Spawning food and growing the snake when food is eaten.
**Exercise**: Generate random food positions. When head reaches food, increase snake length by 1.
**Hint**: Use a simple PRNG or counter-based randomness for food placement.
**Book**: Discovery MB2 - Chapter 13

### Day 91: Snake Game - Part 5 (Timing)
**Topic**: Using timer interrupts for consistent game speed.
**Exercise**: Move the snake every 500ms using a timer interrupt. Increase speed as score increases.
**Hint**: Reduce timer period based on score: `period = base_period - (score * speedup)`.
**Book**: Discovery MB2 - Chapter 13

### Day 92: Snake Game - Part 6 (Polish)
**Topic**: Adding score display, game over screen, and restart functionality.
**Exercise**: Show score as binary on LEDs after game over. Hold both buttons to restart.
**Hint**: Use a state machine: Playing, GameOver, Paused.
**Book**: Discovery MB2 - Chapter 13

### Day 93: Testing & Debugging
**Topic**: Systematic debugging and testing of embedded systems.
**Exercise**: Add debug logging to your snake game. Use `defmt` to trace game state changes.
**Hint**: Log entering/exiting each state, collisions, and score updates.
**Book**: Embedded Rust Book - Chapter 10: Debugging

### Day 94: Code Organization
**Topic**: Structuring embedded Rust projects with modules and traits.
**Exercise**: Refactor your game into modules: `display`, `input`, `game_logic`, `main`. Use traits where appropriate.
**Hint**: Each module should have a clear, single responsibility.
**Book**: Embedded Rust Book - Chapter 19: Design Patterns

### Day 95: Flash Optimization
**Topic**: Reducing binary size for resource-constrained devices.
**Exercise**: Enable LTO and opt-level="z" in Cargo.toml. Compare binary sizes before/after.
**Hint**: Release builds with optimization can be 10x smaller than debug builds!
**Book**: Embedded Rust Book - Chapter 20: Optimization

### Day 96: Alternative Project - Weather Station
**Topic**: Building a sensor data logger with display.
**Exercise**: Read accelerometer as "activity sensor". Display activity level and elapsed time on LEDs. Log to serial.
**Hint**: Calculate magnitude: `sqrt(x² + y² + z²)`. High magnitude = activity.
**Book**: Discovery MB2 - Chapters 10, 12

### Day 97: Alternative Project - Reaction Timer
**Topic**: Building a precision reaction time measurement device.
**Exercise**: Random delay → LED lights → measure time until button press. Display result in milliseconds via serial.
**Hint**: Use a high-resolution timer (1 MHz) for accurate timing.
**Book**: Discovery MB2 - Chapters 6, 7, 10

### Day 98: Documentation & Sharing
**Topic**: Writing embedded documentation and preparing code for sharing.
**Exercise**: Add doc comments to your project. Generate docs with `cargo doc`. Write a README with hardware setup instructions.
**Hint**: Good docs include: required hardware, pin connections, build instructions, and usage examples.
**Book**: Embedded Rust Book - Chapter 21

### Day 99: Review & Next Steps
**Topic**: Reflecting on your journey and planning continued learning.
**Exercise**: List 5 things you learned and 3 areas to explore deeper. Share your project on GitHub!
**Hint**: Consider exploring: RTOS (FreeRTOS), Bluetooth (nRF Softdevice), Embassy async framework, different MCU platforms.
**Book**: Embedded Rust Book - Conclusion | [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)

---

## 🎓 Congratulations!

You've completed 99 days of Embedded Rust! You now have practical experience with:
- ✅ Bare-metal programming fundamentals
- ✅ Memory-mapped I/O and register manipulation
- ✅ Interrupt-driven programming
- ✅ Serial communication protocols (UART, I2C)
- ✅ Sensor integration and data processing
- ✅ Real-time constraints and timing
- ✅ Complete project development

## 📚 Recommended Next Steps

1. **Explore Embassy Framework**: Modern async/await for embedded Rust
2. **Learn Bluetooth**: Nordic nRF Softdevice for BLE applications
3. **Different Platforms**: Try STM32, ESP32, or RP2040
4. **RTOS Integration**: FreeRTOS or Zephyr with Rust bindings
5. **Advanced Topics**: Bootloaders, OTA updates, secure boot
6. **Community**: Join the Rust Embedded Matrix chat and contribute!

## 🛠️ Hardware Shopping List

To complete this curriculum, you need:
- **BBC micro:bit v2** (~$20) - Required for all exercises
- **USB cable** (micro-USB) - Usually included with micro:bit
- Optional: Breadboard, jumper wires, additional sensors for extensions

## 📖 Key Resources

- [Discovery MB2 Book](https://docs.rust-embedded.org/discovery-mb2/)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [RTIC Framework](https://rtic.rs)
- [Embedded Rust Bookshelf](https://docs.rust-embedded.org)
- [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)

Happy Hacking! 🦀🔧
