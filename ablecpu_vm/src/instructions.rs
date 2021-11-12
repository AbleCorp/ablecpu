
#[derive(Debug)]
pub enum Instruction {
    Load(u64, u64),
    Copy(u64, u64),
    Swap(u64, u64),
    Comp(u64, u64),
    Add(u64, u64),
    Sub(u64, u64),
    Mul(u64, u64),
    Div(u64, u64),
}

enum InstructionSpeed {
    Fast,
    Medium,
    Slow,
    Halt
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


    }
}