#[derive(Debug)]
pub enum Instruction {
    Load(u64, u64, bool, bool, bool, InstructionSpeed),
    Copy(u64, u64, bool, bool, bool, InstructionSpeed),
    Swap(u64, u64, bool, bool, bool, InstructionSpeed),
    Comp(u64, u64, bool, bool, bool, InstructionSpeed),
    Add(u64, u64, bool, bool, bool, InstructionSpeed),
    Sub(u64, u64, bool, bool, bool, InstructionSpeed),
    Mul(u64, u64, bool, bool, bool, InstructionSpeed),
    Div(u64, u64, bool, bool, bool, InstructionSpeed),
}

#[derive(Debug)]
enum InstructionSpeed {
    Fast,
    Medium,
    Slow,
    Halt,
}

impl InstructionSpeed {
    fn from_u8(raw: u8) -> InstructionSpeed {
        match raw & 0b0000_0011 {
            0 => InstructionSpeed::Fast,
            1 => InstructionSpeed::Medium,
            2 => InstructionSpeed::Slow,
            3 => InstructionSpeed::Halt,
        }
    }
}

impl Instruction {
    pub fn from_tuple(tuple: (u8, u64, u64)) -> Instruction {
        let instruction_type = tuple.0 & 0b11100000;
        let do_error_handling = if tuple.0 & 0b00010000 == 0b00010000 {
            true
        } else {
            false
        };

        let halt_if_error = if tuple.0 & 0b00001000 == 0b00001000 {
            true
        } else {
            false
        };

        let do_debug_info = if tuple.0 & 0b00000100 == 0b00000100 {
            true
        } else {
            false
        };

        match instruction_type {
            0b00000000 => Instruction::Load(
                tuple.1,
                tuple.2,
                do_error_handling,
                halt_if_error,
                do_debug_info,
                InstructionSpeed::from_u8(tuple.0),
            ),
            0b00100000 => Instruction::Copy(
                tuple.1,
                tuple.2,
                do_error_handling,
                halt_if_error,
                do_debug_info,
                InstructionSpeed::from_u8(tuple.0),
            ),
            0b01000000 => Instruction::Swap(
                tuple.1,
                tuple.2,
                do_error_handling,
                halt_if_error,
                do_debug_info,
                InstructionSpeed::from_u8(tuple.0),
            ),
            0b01100000 => Instruction::Comp(
                tuple.1,
                tuple.2,
                do_error_handling,
                halt_if_error,
                do_debug_info,
                InstructionSpeed::from_u8(tuple.0),
            ),
            0b10000000 => Instruction::Add(
                tuple.1,
                tuple.2,
                do_error_handling,
                halt_if_error,
                do_debug_info,
                InstructionSpeed::from_u8(tuple.0),
            ),
            0b10100000 => Instruction::Sub(
                tuple.1,
                tuple.2,
                do_error_handling,
                halt_if_error,
                do_debug_info,
                InstructionSpeed::from_u8(tuple.0),
            ),
            0b11000000 => Instruction::Mul(
                tuple.1,
                tuple.2,
                do_error_handling,
                halt_if_error,
                do_debug_info,
                InstructionSpeed::from_u8(tuple.0),
            ),
            0b11100000 => Instruction::Div(
                tuple.1,
                tuple.2,
                do_error_handling,
                halt_if_error,
                do_debug_info,
                InstructionSpeed::from_u8(tuple.0),
            ),
        }
    }
}
