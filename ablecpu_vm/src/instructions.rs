use crate::{arch::Arch, errors::CpuError};

#[derive(Debug)]
pub enum Instruction<T: Arch> {
    NoOp(T, T, bool, bool, bool, InstructionSpeed<T>),
    Load(T, T, bool, bool, bool, InstructionSpeed<T>),
    Copy(T, T, bool, bool, bool, InstructionSpeed<T>),
    Comp(T, T, bool, bool, bool, InstructionSpeed<T>),
    Add(T, T, bool, bool, bool, InstructionSpeed<T>),
    Sub(T, T, bool, bool, bool, InstructionSpeed<T>),
    Mul(T, T, bool, bool, bool, InstructionSpeed<T>),
    Div(T, T, bool, bool, bool, InstructionSpeed<T>),
}

#[derive(Debug)]
pub enum InstructionSpeed<T: Arch> {
    Fast,
    Medium,
    Slow,
    Halt,
    Unknown(T),
}

impl<T: Arch> InstructionSpeed<T> {
    fn from_u8(raw: u8) -> Result<InstructionSpeed<T>, CpuError<T>> {
        match raw & 0b0000_0011 {
            0 => Ok(InstructionSpeed::Fast),
            1 => Ok(InstructionSpeed::Medium),
            2 => Ok(InstructionSpeed::Slow),
            3 => Ok(InstructionSpeed::Halt),
            _ => Err(CpuError::IllegalInstructionSpeed(raw.into())),
        }
    }
}

impl<T: Arch> Instruction<T> {
    pub fn from_tuple(tuple: (u8, T, T)) -> Result<Instruction<T>, CpuError<T>> {
        let instruction_type = tuple.0 & 0b11100000;
        let ignore_errors = if tuple.0 & 0b00010000 == 0b00010000 {
            true
        } else {
            false
        };

        let no_halt_if_error = if tuple.0 & 0b00001000 == 0b00001000 {
            true
        } else {
            false
        };

        let no_debug_info = if tuple.0 & 0b00000100 == 0b00000100 {
            true
        } else {
            false
        };

        match instruction_type {
            0b00000000 => Ok(Instruction::NoOp(
                tuple.1,
                tuple.2,
                ignore_errors,
                no_halt_if_error,
                no_debug_info,
                InstructionSpeed::from_u8(tuple.0)?,
            )),
            0b00100000 => Ok(Instruction::Load(
                tuple.1,
                tuple.2,
                ignore_errors,
                no_halt_if_error,
                no_debug_info,
                InstructionSpeed::from_u8(tuple.0)?,
            )),
            0b01000000 => Ok(Instruction::Copy(
                tuple.1,
                tuple.2,
                ignore_errors,
                no_halt_if_error,
                no_debug_info,
                InstructionSpeed::from_u8(tuple.0)?,
            )),
            0b01100000 => Ok(Instruction::Comp(
                tuple.1,
                tuple.2,
                ignore_errors,
                no_halt_if_error,
                no_debug_info,
                InstructionSpeed::from_u8(tuple.0)?,
            )),
            0b10000000 => Ok(Instruction::Add(
                tuple.1,
                tuple.2,
                ignore_errors,
                no_halt_if_error,
                no_debug_info,
                InstructionSpeed::from_u8(tuple.0)?,
            )),
            0b10100000 => Ok(Instruction::Sub(
                tuple.1,
                tuple.2,
                ignore_errors,
                no_halt_if_error,
                no_debug_info,
                InstructionSpeed::from_u8(tuple.0)?,
            )),
            0b11000000 => Ok(Instruction::Mul(
                tuple.1,
                tuple.2,
                ignore_errors,
                no_halt_if_error,
                no_debug_info,
                InstructionSpeed::from_u8(tuple.0)?,
            )),
            0b11100000 => Ok(Instruction::Div(
                tuple.1,
                tuple.2,
                ignore_errors,
                no_halt_if_error,
                no_debug_info,
                InstructionSpeed::from_u8(tuple.0)?,
            )),
            _ => Err(CpuError::IllegalInstruction(tuple.0.into())),
        }
    }
}
