#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board, hal::Timer};
use panic_halt as _;

// Connecting to the microbit over serial
// Install minicom on mac with `brew install minicom`
// Connect microbit over usb
// You can see the device name with command `ls /dev/cu.usbmodem*`
// The `/dev/` folder is a special unix folder for devices
// In order to connect using minicom you need to add a configuration file

/*
cat > ~/.minirc.dfl << 'EOF'
pu baudrate 115200
pu bits 8
pu parity N
pu stopbits 1
pu rtscts No
pu xonxoff No
EOF
*/

// You can veryfy that this was created correctly useing `cat .minirc.dfl`
// To launch mincom run `minicom -D /dev/cu.usbmodem1202 -b 115200` use the device name here
// This will open the minicom cli
// To interact with it on a mac you need to use the Meta key (which is ESC Key)
// Press ESC + Z to enter the help menu to see all commands available



#[entry]
fn main() -> ! {
    let board = board::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut  pins = board.display_pins;

    pins.row1.set_high().unwrap();
    pins.col1.set_low().unwrap();

    loop {}
}