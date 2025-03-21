# CHIP-8 Emulator in Rust

A simple CHIP-8 emulator implementation written in Rust using SDL2 for graphics and input.

## Overview

CHIP-8 is an interpreted programming language from the 1970s, initially used on the COSMAC VIP and Telmac 1800 microcomputers. This emulator provides a modern implementation of the CHIP-8 virtual machine that can run classic games and programs.

## Features

- Full implementation of the standard CHIP-8 instruction set
- 64x32 monochrome display
- Adjustable emulation speed
- Keyboard input mapping for the 16-key CHIP-8 keypad
- Multiple games included

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- SDL2 development libraries

### Installing SDL2

#### Ubuntu/Debian
```
sudo apt install libsdl2-dev
```

#### macOS
```
brew install sdl2
```

#### Windows
Follow the instructions at: https://github.com/Rust-SDL2/rust-sdl2#windows

## Building

```
cargo build --release
```

## Running

```
cargo run --release <path_to_chip8_rom>
```

Or after building:
```
./target/release/chip8 <path_to_chip8_rom>
```

## Controls

The CHIP-8 uses a 16-key hexadecimal keypad. This emulator maps those keys to the following on your keyboard:

```
CHIP-8 Keypad    Keyboard
+-+-+-+-+        +-+-+-+-+
|1|2|3|C|        |1|2|3|4|
+-+-+-+-+        +-+-+-+-+
|4|5|6|D|        |Q|W|E|R|
+-+-+-+-+  --->  +-+-+-+-+
|7|8|9|E|        |A|S|D|F|
+-+-+-+-+        +-+-+-+-+
|A|0|B|F|        |Z|X|C|V|
+-+-+-+-+        +-+-+-+-+
```

## License

MIT
