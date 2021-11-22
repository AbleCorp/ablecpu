
#[derive(Debug)]
pub enum CpuError {
    IllegalState(u64),
    IllegalInstruction(u64),
    IllegalInstructionSpeed(u64),
    DeviceError(u64),
    AddressNotPopulated(u64),
    TooManyDevices(u64),
}
