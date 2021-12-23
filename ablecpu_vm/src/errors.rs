#[derive(Debug)]
pub enum CpuError<T> {
    IllegalState(T),
    IllegalInstruction(T),
    IllegalInstructionSpeed(T),
    DeviceError(T),
    AddressNotPopulated(T),
    TooManyDevices(T),
}
