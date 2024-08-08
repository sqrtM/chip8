pub mod memory;
use std::sync::{Arc, RwLock};

use crate::{
    gui::{Controller, UserEvent},
    internals::memory::{Ram, Registers},
};
use rand::prelude::*;
use winit::keyboard::Key;

const ON: u32 = 0b00000000_00000000_11111111_11111111;
const OFF: u32 = 0;

#[derive(Debug)]
pub enum Instruction {
    ClearDisplay,
    ReturnFromSubRoutine,
    JumpTo(Address),
    Call(Address),
    SkipIf(Register, Nybble),
    SkipIfNot(Register, Nybble),
    SkipIfRegistersEqual(Register, Register),
    LoadInto(Register, Nybble),
    Add(Register, Nybble),
    LoadIntoRegister(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    AddRegisters(Register, Register),
    Sub(Register, Register),
    ShiftRight(Register, Register),
    SubBorrow(Register, Register),
    ShiftLeft(Register, Register),
    SkipIfNotEqual(Register, Register),
    LoadIntoI(Address),
    JumpV0(Address),
    Random(Register, Nybble),
    Draw(Register, Register, Nybble),
    SkipIfPressed(Register),
    SkipIfNotPressed(Register),
    LoadFromDelay(Register),
    WaitForKey(Register),
    LoadToDelay(Register),
    LoadToSound(Register),
    AddToI(Register),
    LoadSpriteToI(Register),
    LoadBcd(Register),
    LoadToMemory(Register),
    LoadFromMemory(Register),
    Nop,
}

#[derive(PartialEq)]
pub enum DisplayCommand {
    ClearDisplay,
    Draw([u32; 2048]),
}

impl UserEvent for DisplayCommand {
    fn transform(&self, b: &mut [u32]) {
        match self {
            DisplayCommand::ClearDisplay => b.fill(0),
            DisplayCommand::Draw(fb) => {
                let original_width = 64;
                let scale_factor = 20;
                let original_height = 32;
                // Calculate new dimensions
                let new_width = original_width * scale_factor;

                for y in 0..original_height {
                    for x in 0..original_width {
                        // Get the original pixel value
                        let pixel = fb[y * original_width + x];

                        // Calculate the position in the scaled framebuffer
                        for dy in 0..scale_factor {
                            for dx in 0..scale_factor {
                                let new_x = x * scale_factor + dx;
                                let new_y = y * scale_factor + dy;

                                // Set the pixel in the scaled framebuffer
                                b[new_y * new_width + new_x] = pixel;
                            }
                        }
                    }
                }
            }
        }
    }
}

struct SpriteData(Vec<Nybble>);

pub struct Sprite {
    x: u8,
    y: u8,
    data: SpriteData, // max length of 15 (0xF)
}

#[derive(PartialEq, Debug)]
pub enum Button {
    B0 = 0x00,
    B1 = 0x01,
    B2 = 0x02,
    B3 = 0x03,
    B4 = 0x04,
    B5 = 0x05,
    B6 = 0x06,
    B7 = 0x07,
    B8 = 0x08,
    B9 = 0x09,
    BA = 0x0A,
    BB = 0x0B,
    BC = 0x0C,
    BD = 0x0D,
    BE = 0x0E,
    BF = 0x0F,
}

impl Button {
    fn to_button(k: &Key) -> Option<Self> {
        match k {
            Key::Named(_) => None,
            Key::Character(c) => match c.as_str() {
                "A" => Some(Button::BA),
                "B" => Some(Button::BB),
                "C" => Some(Button::BC),
                "D" => Some(Button::BD),
                "E" => Some(Button::BE),
                "F" => Some(Button::BF),
                "1" => Some(Button::B1),
                "2" => Some(Button::B2),
                "3" => Some(Button::B3),
                "4" => Some(Button::B4),
                "5" => Some(Button::B5),
                "6" => Some(Button::B6),
                "7" => Some(Button::B7),
                "8" => Some(Button::B8),
                "9" => Some(Button::B9),
                "0" => Some(Button::B0),
                &_ => None,
            },
            Key::Unidentified(_) => None,
            Key::Dead(_) => None,
        }
    }

    fn from_u8(n: u8) -> Self {
        match n {
            0x0 => Button::B0,
            0x1 => Button::B1,
            0x2 => Button::B2,
            0x3 => Button::B3,
            0x4 => Button::B4,
            0x5 => Button::B5,
            0x6 => Button::B6,
            0x7 => Button::B7,
            0x8 => Button::B8,
            0x9 => Button::B9,
            0xA => Button::BA,
            0xB => Button::BB,
            0xC => Button::BC,
            0xD => Button::BD,
            0xE => Button::BE,
            0xF => Button::BF,
            _ => panic!("{n}"),
        }
    }
}

pub struct Chip8Controller(pub Arc<RwLock<Controller>>);

impl Chip8Controller {
    pub fn keys_to_buttons(&self) -> Vec<Option<Button>> {
        self.0
            .read()
            .unwrap()
            .pressing
            .iter()
            .map(Button::to_button)
            .collect()
    }
}

pub struct Chip8 {
    pub registers: Registers,
    pub memory: Ram,
    pub controller: Chip8Controller,
    pub frame_buffer: [u32; 2048],
    pub status: InstructionResult,
}

impl Chip8 {
    pub fn new(controller: Arc<RwLock<Controller>>) -> Self {
        let mut c8 = Chip8 {
            registers: Registers::default(),
            memory: Ram::default(),
            controller: Chip8Controller(controller),
            frame_buffer: [0; 2048],
            status: InstructionResult::Success,
        };
        c8.memory.load("./data/inital_ram_data.chip8");
        c8
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Register {
    V0 = 0x00,
    V1 = 0x01,
    V2 = 0x02,
    V3 = 0x03,
    V4 = 0x04,
    V5 = 0x05,
    V6 = 0x06,
    V7 = 0x07,
    V8 = 0x08,
    V9 = 0x09,
    VA = 0x0A,
    VB = 0x0B,
    VC = 0x0C,
    VD = 0x0D,
    VE = 0x0E,
    VF = 0x0F,
}

type Address = u16;
type Nybble = u8;

impl Register {
    fn from_nybble(i: Nybble) -> Self {
        match i {
            0x00 => Register::V0,
            0x01 => Register::V1,
            0x02 => Register::V2,
            0x03 => Register::V3,
            0x04 => Register::V4,
            0x05 => Register::V5,
            0x06 => Register::V6,
            0x07 => Register::V7,
            0x08 => Register::V8,
            0x09 => Register::V9,
            0x0A => Register::VA,
            0x0B => Register::VB,
            0x0C => Register::VC,
            0x0D => Register::VD,
            0x0E => Register::VE,
            0x0F => Register::VF,
            _ => panic!("{:?}", i),
        }
    }
}

#[derive(PartialEq)]
pub enum InstructionResult {
    Success,
    Display(DisplayCommand),
    Waiting,
}

impl Chip8 {
    pub fn increment_pc(&mut self, increments: u16) {
        self.registers.pc += 2 * increments
    }

    pub fn run_instruction(&mut self, i: Instruction) -> Result<InstructionResult, ()> {
        match i {
            Instruction::ClearDisplay => {
                self.increment_pc(1);
                self.frame_buffer.fill(0);
                Ok(InstructionResult::Display(DisplayCommand::ClearDisplay))
            }
            Instruction::ReturnFromSubRoutine => match self.registers.stack.pop() {
                Some(addr) => {
                    self.registers.pc = addr;
                    Ok(InstructionResult::Success)
                }
                None => Err(()),
            },
            Instruction::JumpTo(addr) => {
                self.registers.pc = addr;
                Ok(InstructionResult::Success)
            }
            Instruction::Call(addr) => {
                self.increment_pc(1);
                self.registers.stack.push(self.registers.pc);
                self.registers.pc = addr;
                Ok(InstructionResult::Success)
            }
            Instruction::SkipIf(x, v) => {
                self.increment_pc(if self.read(x) == v { 2 } else { 1 });
                Ok(InstructionResult::Success)
            }
            Instruction::SkipIfNot(x, v) => {
                self.increment_pc(if self.read(x) != v { 2 } else { 1 });
                Ok(InstructionResult::Success)
            }
            Instruction::SkipIfRegistersEqual(x, y) => {
                self.increment_pc(if self.read(x) == self.read(y) { 2 } else { 1 });
                Ok(InstructionResult::Success)
            }
            Instruction::LoadInto(x, v) => {
                self.write(x, v);
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::Add(x, v) => {
                self.write(x, self.read(x).wrapping_add(v));
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::LoadIntoRegister(x, y) => {
                self.write(x, self.read(y));
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::Or(x, y) => {
                self.write(x, self.read(x) | self.read(y));
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::And(x, y) => {
                self.write(x, self.read(x) & self.read(y));
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::Xor(x, y) => {
                self.write(x, self.read(x) ^ self.read(y));
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::AddRegisters(x, y) => {
                let x_plus_y = self.read(x).overflowing_add(self.read(y));
                self.write(x, x_plus_y.0);
                self.write(Register::VF, if x_plus_y.1 { 1 } else { 0 });
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::Sub(x, y) => {
                let x_minus_y = self.read(x).wrapping_sub(self.read(y));
                self.write(x, x_minus_y);
                self.write(
                    Register::VF,
                    if self.read(x) > self.read(y) { 0 } else { 1 },
                );
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::ShiftRight(x, y) => {
                let y_shr = self.read(y) >> 1;
                self.write(x, y_shr);
                self.write(Register::VF, if self.read(y) & 1 == 1 { 1 } else { 0 });
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::SubBorrow(x, y) => {
                let y_minus_x = self.read(y).wrapping_sub(self.read(x));
                self.write(x, y_minus_x);
                self.write(
                    Register::VF,
                    if self.read(y) > self.read(x) { 0 } else { 1 },
                );
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::ShiftLeft(x, y) => {
                let y_shl = self.read(y) << 1;
                self.write(x, y_shl);
                self.write(
                    Register::VF,
                    if (self.read(y) & (1 << 7)) != 0 { 1 } else { 0 },
                );
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::SkipIfNotEqual(x, y) => {
                self.increment_pc(if self.read(x) != self.read(y) { 2 } else { 1 });
                Ok(InstructionResult::Success)
            }
            Instruction::LoadIntoI(addr) => {
                self.write_i(addr);
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::JumpV0(addr) => {
                self.registers.pc = addr + self.read(Register::V0) as u16;
                Ok(InstructionResult::Success)
            }
            Instruction::Random(x, v) => {
                self.write(x, rand::thread_rng().gen::<u8>() & v);
                self.increment_pc(1);
                Ok(InstructionResult::Success)
            }
            Instruction::Draw(x, y, l) => {
                self.increment_pc(1);
                let s = Sprite {
                    x: self.read(x),
                    y: self.read(y),
                    data: SpriteData(
                        self.memory
                            .0
                            .iter()
                            .skip(self.read_i() as usize)
                            .take(l as usize)
                            .cloned()
                            .collect(),
                    ),
                };

                for (i, byte) in s.data.0.iter().enumerate() {
                    let y_coord = (s.y as usize + i) % 32;
                    let width = 64;
                    for j in (1..8).rev() {
                        let x_coord = (s.x as usize + (7 - j)) % width;
                        let index: usize = y_coord * width + x_coord;
                        self.frame_buffer[index] = if (byte >> j) & 1 == 1 {
                            if self.frame_buffer[index] ^ ON != ON {
                                self.write(Register::VF, 1)
                            }
                            self.frame_buffer[index] ^ ON
                        } else {
                            OFF
                        };
                    }
                }

                Ok(InstructionResult::Display(DisplayCommand::Draw(
                    self.frame_buffer,
                )))
            }
            Instruction::SkipIfPressed(x) => {
                self.increment_pc(
                    if self
                        .controller
                        .keys_to_buttons()
                        .contains(&Some(Button::from_u8(self.read(x))))
                    {
                        2
                    } else {
                        1
                    },
                );
                Ok(InstructionResult::Success)
            }
            Instruction::SkipIfNotPressed(x) => {
                self.increment_pc(
                    if !self
                        .controller
                        .keys_to_buttons()
                        .contains(&Some(Button::from_u8(self.read(x))))
                    {
                        2
                    } else {
                        1
                    },
                );
                Ok(InstructionResult::Success)
            }
            Instruction::LoadFromDelay(x) => {
                self.increment_pc(1);
                self.write(x, self.read_delay());
                Ok(InstructionResult::Success)
            }
            Instruction::WaitForKey(x) => {
                if self.status != InstructionResult::Waiting {
                    match self.controller.0.try_write() {
                        Ok(mut c) => {
                            c.last_released = None;
                            Ok(InstructionResult::Waiting)
                        }
                        Err(_) => Err(()),
                    }
                } else {
                    let con = Arc::clone(&self.controller.0);
                    let r = con.try_read();
                    match r {
                        Ok(c) => match &c.last_released {
                            Some(b) => {
                                if Button::to_button(b).is_some() {
                                    self.write(x, Button::to_button(b).unwrap() as u8);
                                    Ok(InstructionResult::Success)
                                } else {
                                    Ok(InstructionResult::Waiting)
                                }
                            }
                            None => Ok(InstructionResult::Waiting),
                        },
                        Err(_) => Err(()),
                    }
                }
            }

            Instruction::LoadToDelay(x) => {
                self.increment_pc(1);
                self.write_delay(self.read(x));
                Ok(InstructionResult::Success)
            }
            Instruction::LoadToSound(x) => {
                self.increment_pc(1);
                self.write_sound(self.read(x));
                Ok(InstructionResult::Success)
            }
            Instruction::AddToI(x) => {
                self.increment_pc(1);
                self.write_i(self.read_i() + self.read(x) as u16);
                Ok(InstructionResult::Success)
            }
            Instruction::LoadSpriteToI(x) => {
                self.increment_pc(1);
                // write to I the value of the sprite
                // representing the hex value in x
                self.write_i(self.read(x) as u16 * 5);
                Ok(InstructionResult::Success)
            }
            Instruction::LoadBcd(x) => {
                self.increment_pc(1);
                let i = self.read_i() as usize;
                let _x = self.read(x);
                self.memory.0[i..=i + 2].copy_from_slice(&[_x / 100, (_x % 100) / 10, _x % 10]);
                Ok(InstructionResult::Success)
            }
            Instruction::LoadToMemory(x) => {
                self.increment_pc(1);
                let range = self.read_i() as usize..=(self.read_i() + x as u16) as usize;
                self.memory.0[range.clone()].copy_from_slice(&self.registers.r[0..=x as usize]);
                Ok(InstructionResult::Success)
            }
            Instruction::LoadFromMemory(x) => {
                self.increment_pc(1);
                let range = self.read_i() as usize..=(self.read_i() + x as u16) as usize;
                self.registers.r[0..=x as usize].copy_from_slice(&self.memory.0[range]);
                Ok(InstructionResult::Success)
            }
            Instruction::Nop => Ok(InstructionResult::Success),
        }
    }

    fn read(&self, r: Register) -> u8 {
        self.registers.r[r as usize]
    }

    fn write(&mut self, r: Register, v: u8) {
        self.registers.r[r as usize] = v
    }

    fn read_i(&self) -> u16 {
        self.registers.vi
    }

    fn write_i(&mut self, v: u16) {
        self.registers.vi = v
    }

    fn read_delay(&self) -> u8 {
        self.registers.delay
    }

    fn write_delay(&mut self, v: u8) {
        self.registers.delay = v
    }

    fn write_sound(&mut self, v: u8) {
        self.registers.sound = v
    }
}

pub fn parse_opcode(i: u16) -> Instruction {
    match i {
        0x00E0 => Instruction::ClearDisplay,
        0x00EE => Instruction::ReturnFromSubRoutine,
        0x1000..0x2000 => Instruction::JumpTo(address(i)),
        0x2000..0x3000 => Instruction::Call(address(i)),
        0x3000..0x4000 => Instruction::SkipIf(x_register(i), low_byte(i)),
        0x4000..0x5000 => Instruction::SkipIfNot(x_register(i), low_byte(i)),
        0x5000..0x6000 => Instruction::SkipIfRegistersEqual(x_register(i), y_register(i)),
        0x6000..0x7000 => Instruction::LoadInto(x_register(i), low_byte(i)),
        0x7000..0x8000 => Instruction::Add(x_register(i), low_byte(i)),
        0x8000..0x9000 => match low_nybble(i) {
            0x0000 => Instruction::LoadIntoRegister(x_register(i), y_register(i)),
            0x0001 => Instruction::Or(x_register(i), y_register(i)),
            0x0002 => Instruction::And(x_register(i), y_register(i)),
            0x0003 => Instruction::Xor(x_register(i), y_register(i)),
            0x0004 => Instruction::AddRegisters(x_register(i), y_register(i)),
            0x0005 => Instruction::Sub(x_register(i), y_register(i)),
            0x0006 => Instruction::ShiftRight(x_register(i), y_register(i)),
            0x0007 => Instruction::SubBorrow(x_register(i), y_register(i)),
            0x000E => Instruction::ShiftLeft(x_register(i), y_register(i)),
            _ => Instruction::Nop,
        },
        0x9000..0xA000 => Instruction::SkipIfNotEqual(x_register(i), y_register(i)),
        0xA000..0xB000 => Instruction::LoadIntoI(address(i)),
        0xB000..0xC000 => Instruction::JumpV0(address(i)),
        0xC000..0xD000 => Instruction::Random(x_register(i), low_byte(i)),
        0xD000..0xE000 => Instruction::Draw(x_register(i), y_register(i), low_nybble(i)),
        0xE000..0xF000 => match low_byte(i) {
            0x009E => Instruction::SkipIfPressed(x_register(i)),
            0x00A1 => Instruction::SkipIfNotPressed(x_register(i)),
            _ => Instruction::Nop,
        },
        0xF000..=0xFFFF => match low_byte(i) {
            0x0007 => Instruction::LoadFromDelay(x_register(i)),
            0x000A => Instruction::WaitForKey(x_register(i)),
            0x0015 => Instruction::LoadToDelay(x_register(i)),
            0x0018 => Instruction::LoadToSound(x_register(i)),
            0x001E => Instruction::AddToI(x_register(i)),
            0x0029 => Instruction::LoadSpriteToI(x_register(i)),
            0x0033 => Instruction::LoadBcd(x_register(i)),
            0x0055 => Instruction::LoadToMemory(x_register(i)),
            0x0065 => Instruction::LoadFromMemory(x_register(i)),
            _ => Instruction::Nop,
        },
        _ => Instruction::Nop,
    }
}

fn x_register(i: u16) -> Register {
    Register::from_nybble(((i & 0x0F00) >> 8) as Nybble)
}

fn y_register(i: u16) -> Register {
    Register::from_nybble(((i & 0x00F0) >> 4) as Nybble)
}

fn low_byte(i: u16) -> Nybble {
    (i & 0x00FF) as Nybble
}

fn low_nybble(i: u16) -> Nybble {
    (i & 0x000F) as Nybble
}

fn address(i: u16) -> Address {
    i & 0x0FFF
}
