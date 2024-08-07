pub mod memory;
use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::{
    gui::Controller,
    internals::memory::{Ram, Registers},
};
use rand::prelude::*;

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

pub enum DisplayCommand {
    ClearDisplay,
    Draw(Sprite),
}

struct SpriteData(Vec<Nybble>);

struct Sprite {
    x: u8,
    y: u8,
    data: SpriteData,
}

pub struct Chip8 {
    pub registers: Registers,
    pub memory: Ram,
    pub controller: Arc<RwLock<Controller>>,
}

impl Chip8 {
    pub fn new(controller: Arc<RwLock<Controller>>) -> Self {
        Chip8 {
            registers: Registers::default(),
            memory: Ram::default(),
            controller,
        }
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

pub enum InstructionResult {
    Success,
    Display(DisplayCommand),
}

impl Chip8 {
    pub fn increment_pc(&mut self, increments: u16) {
        self.registers.pc += (2 * increments)
    }

    pub fn run_instruction(&mut self, i: Instruction) -> Result<InstructionResult, ()> {
        match i {
            Instruction::ClearDisplay => {
                self.increment_pc(1);
                Ok(InstructionResult::Display(DisplayCommand::ClearDisplay))
            }
            Instruction::ReturnFromSubRoutine => match self.registers.stack.pop() {
                Some(addr) => {
                    self.registers.pc = addr;
                    self.registers.pc -= 1;
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
                Ok(InstructionResult::Display(DisplayCommand::Draw(Sprite {
                    x: self.read(x),
                    y: self.read(y),
                    data: SpriteData(self.memory.0.iter().skip(l as usize).cloned().collect()),
                })))
            }
            Instruction::SkipIfPressed(_) => todo!(),
            Instruction::SkipIfNotPressed(_) => todo!(),
            Instruction::LoadFromDelay(x) => {
                self.write(x, self.read_delay());
                Ok(InstructionResult::Success)
            }
            Instruction::WaitForKey(_) => todo!(),
            Instruction::LoadToDelay(x) => {
                self.write_delay(self.read(x));
                Ok(InstructionResult::Success)
            }
            Instruction::LoadToSound(x) => {
                self.write_sound(self.read(x));
                Ok(InstructionResult::Success)
            }
            Instruction::AddToI(x) => {
                self.write_i(self.read_i() + self.read(x) as u16);
                Ok(InstructionResult::Success)
            }
            Instruction::LoadSpriteToI(x) => {
                // write to I the value of the sprite
                // representing the hex value in x
                self.write_i(self.read(x) as u16 * 5);
                Ok(InstructionResult::Success)
            }
            Instruction::LoadBcd(x) => {
                // TO TEST
                let mut bcd: u16 = 0;
                let mut shift = 0;

                let mut num = self.read(x);

                while num > 0 {
                    let digit = num % 10; // Get the last digit
                    bcd |= (digit as u16) << shift; // Shift and combine into BCD
                    num /= 10; // Remove the last digit
                    shift += 4; // Move to the next BCD digit
                }
                // ?? ? ?? ? ?? ? ?? ? ?? ? ?? ? ?? ? ??
                self.write_i(bcd);
                Ok(InstructionResult::Success)
            }
            Instruction::LoadToMemory(x) => {
                let range = self.read_i() as usize..(self.read_i() + x as u16) as usize;
                self.memory.0[range].copy_from_slice(&self.registers.r[0..x as usize]);
                Ok(InstructionResult::Success)
            }
            Instruction::LoadFromMemory(x) => {
                let range = self.read_i() as usize..(self.read_i() + x as u16) as usize;
                self.registers.r[0..x as usize].copy_from_slice(&self.memory.0[range]);
                Ok(InstructionResult::Success)
            }
            Instruction::Nop => Ok(InstructionResult::Success),
        }
    }

    fn read(&self, r: Register) -> u8 {
        self.registers.r[r as usize]
    }

    fn write(&mut self, r: Register, v: u8) -> () {
        self.registers.r[r as usize] = v
    }

    fn read_i(&self) -> u16 {
        self.registers.vi
    }

    fn write_i(&mut self, v: u16) -> () {
        self.registers.vi = v
    }

    fn read_delay(&self) -> u8 {
        self.registers.delay
    }

    fn write_delay(&mut self, v: u8) -> () {
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
            0x0000 => Instruction::LoadIntoRegister(y_register(i), x_register(i)),
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
    Register::from_nybble(((i & 0x0F00) >> 12) as Nybble)
}

fn y_register(i: u16) -> Register {
    Register::from_nybble(((i & 0x00F0) >> 8) as Nybble)
}

fn hi_byte(i: u16) -> Nybble {
    ((i >> 8) & 0xFF00) as Nybble
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
