pub enum CpuError {
    IllegalInstruction(u64),
    IllegalInstructionSpeed(u64),
    DeviceError(u64),
}
