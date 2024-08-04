use super::memory::{Ram, Registers};

enum InstructionResult {
    Clear(DisplayCommand),
    ReturnFromSubRoutine,
    JumpTo(u16),
    Call(u16),
    SkipIf(Register, u8),
    SkipIfNot(Register, u8),
    SkipIfRegistersEqual(Register, Register),
    LoadInto(Register, u8),
    Add(Register, u8),
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
    Random(Register, u8),
    Draw(DisplayCommand),
    SkipIfPressed(Register),
    SkipIfNotPressed(Register),
    LoadFromDelay(Register),
    LoadFromKey(Register),
    LoadToDelay(Register),
    LoadToSound(Register),
    AddToI(Register),
    LoadSpriteToI(Register),
    LoadBcd(Register),
    LoadToMemory(Register),
    LoadFromMemory(Register),
    Nop,
}

enum DisplayCommand {
    ClearDisplay,
    Draw(Sprite),
}

struct SpriteData(Vec<u8>);

struct Sprite {
    x: u16,
    y: u16,
    data: SpriteData,
}

pub struct CPU {
    registers: Registers,
    memory: Ram,
}

enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

type Address = u16;

impl Register {
    fn from_u8(i: u8) -> Self {
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
            _ => panic!(),
        }
    }
}

impl CPU {
    fn run_instruction(&mut self, i: InstructionResult) {}
}

fn parse_instruction(i: u16) -> InstructionResult {
    match i {
        0x00E0 => InstructionResult::Clear(DisplayCommand::ClearDisplay), // Clear Display
        0x00EE => InstructionResult::ReturnFromSubRoutine, // Return from a subroutine.
        0x1000..0x2000 => InstructionResult::JumpTo(address(i)),
        0x2000..0x3000 => InstructionResult::Call(address(i)),
        0x3000..0x4000 => InstructionResult::SkipIf(x_register(i), low_byte(i)),
        0x4000..0x5000 => InstructionResult::SkipIfNot(x_register(i), low_byte(i)),
        0x5000..0x6000 => InstructionResult::SkipIfRegistersEqual(x_register(i), y_register(i)),
        0x6000..0x7000 => InstructionResult::LoadInto(x_register(i), low_byte(i)),
        0x7000..0x8000 => InstructionResult::Add(x_register(i), low_byte(i)),
        0x8000..0x9000 => match low_nybble(i) {
            0x0000 => InstructionResult::LoadIntoRegister(y_register(i), x_register(i)),
            0x0001 => InstructionResult::Or(x_register(i), y_register(i)),
            0x0002 => InstructionResult::And(x_register(i), y_register(i)),
            0x0003 => InstructionResult::Xor(x_register(i), y_register(i)),
            0x0004 => InstructionResult::AddRegisters(x_register(i), y_register(i)),
            0x0005 => InstructionResult::Sub(x_register(i), y_register(i)),
            0x0006 => InstructionResult::ShiftRight(x_register(i), y_register(i)),
            0x0007 => InstructionResult::SubBorrow(x_register(i), y_register(i)),
            0x000E => InstructionResult::ShiftLeft(x_register(i), y_register(i)),
            _ => InstructionResult::Nop,
        },
        0x9000..0xA000 => InstructionResult::SkipIfNotEqual(x_register(i), y_register(i)),
        0xA000..0xB000 => InstructionResult::LoadIntoI(address(i)),
        0xB000..0xC000 => InstructionResult::JumpV0(address(i)),
        0xC000..0xD000 => InstructionResult::Random(x_register(i), low_byte(i)),
        0xD000..0xE000 => InstructionResult::Draw(DisplayCommand::Draw(())),
        0xE000..0xF000 => InstructionResult::Nop,
        0xF000..=0xFFFF => InstructionResult::Nop,
        _ => InstructionResult::Nop,
    }
}

fn x_register(i: u16) -> Register {
    Register::from_u8((i & 0x0F00) as u8)
}

fn y_register(i: u16) -> Register {
    Register::from_u8((i & 0x00F0) as u8)
}

fn hi_byte(i: u16) -> u8 {
    ((i >> 8) & 0xFF00) as u8
}

fn low_byte(i: u16) -> u8 {
    (i & 0x00FF) as u8
}

fn low_nybble(i: u16) -> u8 {
    (i & 0x000F) as u8
}

fn address(i: u16) -> Address {
    (i & 0x0FFF) as u16
}
