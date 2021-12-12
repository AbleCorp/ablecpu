#[derive(Debug)]
pub enum Cpu64Error {
    IllegalState(u64),
    IllegalInstruction(u64),
    IllegalInstructionSpeed(u64),
    DeviceError(u64),
    AddressNotPopulated(u64),
    TooManyDevices(u64),
}
