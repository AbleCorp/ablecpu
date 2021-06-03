#[derive(Debug)]
pub enum CPUError{
    IllegalInstruction(String),
    OutOfInstructions(String),
    IllegalArguments(String),
    IllegalAddressLoad(String),
    IllegalAddressPush(String)
}