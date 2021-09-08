
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