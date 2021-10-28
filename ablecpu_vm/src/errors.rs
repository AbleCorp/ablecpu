#[derive(Debug)]
pub enum CPUError{
    IllegalInstruction,
    OutOfInstructions,
    IllegalArguments,
    IllegalAddressLoad,
    IllegalAddressPush
}