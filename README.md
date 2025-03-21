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

This emulator uses a direct 1:1 mapping of hexadecimal keys to your keyboard:

```
CHIP-8 Key    Keyboard Key
-----------   ------------
     0             0
     1             1
     2             2
     3             3
     4             4
     5             5
     6             6
     7             7
     8             8
     9             9
     A             A
     B             B
     C             C
     D             D
     E             E
     F             F
```

## CHIP-8 ROMs

You can find a collection of CHIP-8 ROMs at: https://github.com/netpro2k/Chip8

This repository contains many classic CHIP-8 games including:
- Pong
- Space Invaders
- Tetris
- Breakout
- And many more...

## License

MIT
