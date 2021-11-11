
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

impl Instruction {
    pub fn from_tuple(tuple: (u8, u64, u64)) -> Instruction {
        match tuple {
            (0, a, b) => Instruction::Load(a, b),
            (1, a, b) => Instruction::Copy(a, b),
}