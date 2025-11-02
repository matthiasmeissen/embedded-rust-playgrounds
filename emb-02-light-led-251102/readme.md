
# Steps

## Setup

### .cargo/config.toml
Adds basic variables to make writing cli commands easier
Includes shorthand for target and linker arguments

### Embed.toml
Also includes things used for compiling
Like board name and which debugger to use

### .vscode/settings.json
The rust analyzer does compile the project in the background a checks for errors.
But since we are in a target that is not our machine but the microcontroller, it will complain
We need to tell it that we do not want to check for all targets 
And then explicitly specify what out target will be

### memory.x
This includes the memory leayout of the microcontroller
Where should I add this file.

### Panic Halt
Since oir program is running on a microcontroller that does nothing else
We need to be really precise on what happens when the program panics
For programs than run on a desktop os, we exit the program
But for that case there is not real thing above it

### Dependencies
We need a lot of dependencies to make thsi run:
- cortex_m (Not specifically imported though)
- cortex_m_rt
- microbit
- panic_halt (Important)


## Building

When we run `cargo build` the program should compile.

Alternatively you can run `cargo build --target thumbv7em-none-eabihf` to be more specific.
But since we have specified the target in the .cargo/config.toml this is not neceassary.

It will be located in `target/thumbv7em-none-eabihf/debug/program-name`

### Viewing the binary file
To do this you need some additional tools:
- LLVM Tools from rustup components
- Cargo binutils is a subcommand

There is a long command to view the file:

`cargo readobj --target thumbv7em-none-eabihf --bin program-name -- --file-header`


## Flashing

### Adding cargo embedd
This is a command that lets us send code to the microcontroller 
This is done by talking to the secondary controller on the microbit since we can not acces the arm chip directly

To flash it to the controller we must use:
`cargo embed --chip nrf52833_xxAA`

Tho simplify this we have added the chip details to the Embed.toml file
Which means we can write: `cargo embed`

## Debugging

There are two types of debugging: 
- RTT (Realtime Transfer) which sends message between controller and computer
- GDB which is a full blown debugger and can be very complex

For RTT you need to install a special crate.
For GDB you need to run `brew install arm-none-eabi-gdb` on a mac

To enable usage we need to adjust the Embed.toml file.

There is a flag called halt_afterwards = true
This means that when we flash the program it first stops, 
so it does not enter the infinite loop directly and we can debug things.

So after writing `cargo embed` cargo is holding the program so we can connect gdb to it.
It says that GDB stub listening at 127.0.0.1:1337

### Debug with new GDB in Terminal

To access this we need to open a new terminal and write:
`arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/light-led`

This will print some text but not really do someting
We first have to connect to the server:
`target remote :1337`

Then we can run debug commands.

`break main` set a breakpoint in main
`continue` go the the next step
`quit` close the debugger

There are many more things you can do in that.
