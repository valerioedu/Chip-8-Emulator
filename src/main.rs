use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

use rand;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

const MEMORY_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const START_ADDRESS: u16 = 0x200;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

struct Chip8 {
    pc: u16,
    i: u16,
    sp: u8,
    v: [u8; REGISTER_COUNT],
    memory: [u8; MEMORY_SIZE],
    stack: [u16; STACK_SIZE],
    display: [bool; CHIP8_WIDTH * CHIP8_HEIGHT],
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; 16],
}

impl Chip8 {
    fn new() -> Self {
        let mut chip8 = Chip8 {
            pc: START_ADDRESS,
            i: 0,
            sp: 0,
            v: [0; REGISTER_COUNT],
            memory: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
            display: [false; CHIP8_WIDTH * CHIP8_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; 16],
        };

        for (i, &font) in FONTSET.iter().enumerate() {
            chip8.memory[i] = font;
        }

        chip8
    }

    fn ResetDisplay(&mut self) {
        self.display = [false; CHIP8_WIDTH * CHIP8_HEIGHT];
    }

    fn LoadRom(&mut self, rom_path: &str) {
        let mut file = File::open(rom_path)
            .unwrap_or_else(|_| panic!("Failed to open file: {}", rom_path));
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .unwrap_or_else(|_| panic!("Failed to read file: {}", rom_path));
        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[START_ADDRESS as usize + i] = byte;
        }
    }
    
    fn SetKey(&mut self, key: usize, state: bool) {
        if key < 16 {
            self.keys[key] = state;
        }
    }

    fn cycle(&mut self) {
        let opcode = (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;

        let nibbles = (
            ((opcode & 0xF000) >> 12),
            ((opcode & 0x0F00) >> 8),
            ((opcode & 0x00F0) >> 4),
            (opcode & 0x000F),
        );

        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => {
                self.ResetDisplay();
            }

            (0x0, 0x0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            (0x1, _, _, _) => {
                self.pc = nnn;
            }

            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }

            (0x3, _, _, _) => {
                if self.v[x] == kk {
                    self.pc += 2;
                }
            }

            (0x4, _, _, _) => {
                if self.v[x] != kk {
                    self.pc += 2;
                }
            }

            (0x5, _, _, 0x0) => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }

            (0x6, _, _, _) => {
                self.v[x] = kk;
            }

            (0x7, _, _, _) => {
                self.v[x] = self.v[x].wrapping_add(kk);
            }

            (0x8, _, _, 0x0) => {
                self.v[x] = self.v[y];
            }

            (0x8, _, _, 0x1) => {
                self.v[x] |= self.v[y];
            }

            (0x8, _, _, 0x2) => {
                self.v[x] &= self.v[y];
            }

            (0x8, _, _, 0x3) => {
                self.v[x] ^= self.v[y];
            }

            (0x8, _, _, 0x4) => {
                let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = sum;
                self.v[0xF] = if carry { 1 } else { 0 };
            }

            (0x8, _, _, 0x5) => {
                self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            }

            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            }

            (0x8, _, _, 0x7) => {
                self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            }

            (0x8, _, _, 0xE) => {
                self.v[0xF] = (self.v[x] & 0x80) >> 7;
                self.v[x] <<= 1;
            }

            (0x9, _, _, 0x0) => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }

            (0xA, _, _, _) => {
                self.i = nnn;
            }

            (0xB, _, _, _) => {
                self.pc = (self.v[0] as u16).wrapping_add(nnn);
            }

            (0xC, _, _, _) => {
                let rand_byte = rand::random::<u8>();
                self.v[x] = rand_byte & kk;
            }

            (0xD, _, _, _) => {
                let height = nibbles.3 as u8;
                self.v[0xF] = 0;

                for row in 0..height {
                    let sprite_byte = self.memory[(self.i + row as u16) as usize];
                    for col in 0..8 {
                        if (sprite_byte & (0x80 >> col)) != 0 {
                            let x_coord = (self.v[x] as usize + col as usize) % CHIP8_WIDTH;
                            let y_coord = (self.v[y] as usize + row as usize) % CHIP8_HEIGHT;
                            let idx = x_coord + y_coord * CHIP8_WIDTH;

                            if self.display[idx] {
                                self.v[0xF] = 1;
                            }
                            self.display[idx] ^= true;
                        }
                    }
                }
            }

            (0xE, _, 0x9, 0xE) => {
                if self.keys[self.v[x] as usize] {
                    self.pc += 2;
                }
            }

            (0xE, _, 0xA, 0x1) => {
                if !self.keys[self.v[x] as usize] {
                    self.pc += 2;
                }
            }

            (0xF, _, 0x0, 0x7) => {
                self.v[x] = self.delay_timer;
            }

            (0xF, _, 0x0, 0xA) => {
                let pressed_key = self
                    .keys
                    .iter()
                    .enumerate()
                    .find(|(_, &pressed)| pressed)
                    .map(|(idx, _)| idx);

                if let Some(key_index) = pressed_key {
                    self.v[x] = key_index as u8;
                } else {
                    self.pc -= 2;
                }
            }

            (0xF, _, 0x1, 0x5) => {
                self.delay_timer = self.v[x];
            }

            (0xF, _, 0x1, 0x8) => {
                self.sound_timer = self.v[x];
            }

            (0xF, _, 0x1, 0xE) => {
                self.i = self.i.wrapping_add(self.v[x] as u16);
            }

            (0xF, _, 0x2, 0x9) => {
                self.i = (self.v[x] as u16) * 5;
            }

            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = self.v[x] / 100;
                self.memory[self.i as usize + 1] = (self.v[x] % 100) / 10;
                self.memory[self.i as usize + 2] = self.v[x] % 10;
            }

            (0xF, _, 0x5, 0x5) => {
                for reg_index in 0..=x {
                    self.memory[self.i as usize + reg_index] = self.v[reg_index];
                }
            }

            (0xF, _, 0x6, 0x5) => {
                for reg_index in 0..=x {
                    self.v[reg_index] = self.memory[self.i as usize + reg_index];
                }
            }

            _ => {
                eprintln!("Unrecognized opcode: {:#X}", opcode);
            }
        }
    }
}

fn DrawScreen(canvas: &mut WindowCanvas, chip8: &Chip8, scale: u32) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for y in 0..CHIP8_HEIGHT {
        for x in 0..CHIP8_WIDTH {
            if chip8.display[x + y * CHIP8_WIDTH] {
                let _ = canvas.fill_rect(Rect::new(
                    (x as i32) * scale as i32,
                    (y as i32) * scale as i32,
                    scale,
                    scale,
                ));
            }
        }
    }
    canvas.present();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_chip8_rom>", args[0]);
        return;
    }

    let rom_path = &args[1];

    let mut chip8 = Chip8::new();
    chip8.LoadRom(rom_path);

    let sdl_context = sdl2::init().expect("Failed to init SDL2");
    let video_subsystem = sdl_context.video().expect("Failed to create SDL video subsystem");

    let window = video_subsystem
        .window("CHIP-8 Emulator in Rust", (CHIP8_WIDTH as u32) * 10, (CHIP8_HEIGHT as u32) * 10)
        .position_centered()
        .resizable()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("Failed to get window's canvas");

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    let desired_fps = 600.0;
    let frame_duration = Duration::from_secs_f64(1.0 / desired_fps);

    let mut last_timer_update = Instant::now();
    let timer_interval = Duration::from_millis(16); // 60Hz

    'running: loop {
        let start_time = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }

                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Num0 => chip8.SetKey(0x0, true),
                    Keycode::Num1 => chip8.SetKey(0x1, true),
                    Keycode::Num2 => chip8.SetKey(0x2, true),
                    Keycode::Num3 => chip8.SetKey(0x3, true),
                    Keycode::Num4 => chip8.SetKey(0x4, true),
                    Keycode::Num5 => chip8.SetKey(0x5, true),
                    Keycode::Num6 => chip8.SetKey(0x6, true),
                    Keycode::Num7 => chip8.SetKey(0x7, true),
                    Keycode::Num8 => chip8.SetKey(0x8, true),
                    Keycode::Num9 => chip8.SetKey(0x9, true),
                    Keycode::A => chip8.SetKey(0xA, true),
                    Keycode::B => chip8.SetKey(0xB, true),
                    Keycode::C => chip8.SetKey(0xC, true),
                    Keycode::D => chip8.SetKey(0xD, true),
                    Keycode::E => chip8.SetKey(0xE, true),
                    Keycode::F => chip8.SetKey(0xF, true),
                    _ => {}
                },

                Event::KeyUp {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Num0 => chip8.SetKey(0x0, false),
                    Keycode::Num1 => chip8.SetKey(0x1, false),
                    Keycode::Num2 => chip8.SetKey(0x2, false),
                    Keycode::Num3 => chip8.SetKey(0x3, false),
                    Keycode::Num4 => chip8.SetKey(0x4, false),
                    Keycode::Num5 => chip8.SetKey(0x5, false),
                    Keycode::Num6 => chip8.SetKey(0x6, false),
                    Keycode::Num7 => chip8.SetKey(0x7, false),
                    Keycode::Num8 => chip8.SetKey(0x8, false),
                    Keycode::Num9 => chip8.SetKey(0x9, false),
                    Keycode::A => chip8.SetKey(0xA, false),
                    Keycode::B => chip8.SetKey(0xB, false),
                    Keycode::C => chip8.SetKey(0xC, false),
                    Keycode::D => chip8.SetKey(0xD, false),
                    Keycode::E => chip8.SetKey(0xE, false),
                    Keycode::F => chip8.SetKey(0xF, false),
                    _ => {}
                },
                _ => {}
            }
        }

        chip8.cycle();

        DrawScreen(&mut canvas, &chip8, 10);

        if last_timer_update.elapsed() >= timer_interval {
            last_timer_update = Instant::now();
        }

        let elapsed = Instant::now().duration_since(start_time);
        
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}
