use crate::errors::CpuError;

#[derive(Debug)]
pub enum Instruction {
    NoOp(u64, u64, bool, bool, bool, InstructionSpeed),
    Load(u64, u64, bool, bool, bool, InstructionSpeed),
    Copy(u64, u64, bool, bool, bool, InstructionSpeed),
    Comp(u64, u64, bool, bool, bool, InstructionSpeed),
    Add(u64, u64, bool, bool, bool, InstructionSpeed),
    Sub(u64, u64, bool, bool, bool, InstructionSpeed),
    Mul(u64, u64, bool, bool, bool, InstructionSpeed),
    Div(u64, u64, bool, bool, bool, InstructionSpeed),
}

#[derive(Debug)]
pub enum InstructionSpeed {
    Fast,
    Medium,
    Slow,
    Halt,
}

impl InstructionSpeed {
    fn from_u8(raw: u8) -> Result<InstructionSpeed, CpuError> {
        match raw & 0b0000_0011 {
            0 => Ok(InstructionSpeed::Fast),
            1 => Ok(InstructionSpeed::Medium),
            2 => Ok(InstructionSpeed::Slow),
            3 => Ok(InstructionSpeed::Halt),
            _ => Err(CpuError::IllegalInstructionSpeed(raw as u64)),
        }
    }
}

impl Instruction {
    pub fn from_tuple(tuple: (u8, u64, u64)) -> Result<Instruction, CpuError> {
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
            _ => Err(CpuError::IllegalInstruction(tuple.0 as u64)),
        }
    }
}
