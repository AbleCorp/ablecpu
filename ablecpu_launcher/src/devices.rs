use std::ops::RangeInclusive;

use ablecpu_vm::{errors::CpuError, Device};

pub(crate) struct TerminalOut {}

pub(crate) struct TerminalIn {}

impl Device for TerminalOut {
    fn get_address_space(&self) -> RangeInclusive<u64> {
        131073..=131073
    }

    fn load(&self, address: u64) -> Result<u64, CpuError> {
        println!();
        Ok(0)
    }

    fn push(&self, address: u64, value: u64) -> Result<(), CpuError> {
        print!(
            "{}",
            match std::str::from_utf8(&value.to_be_bytes()) {
                Ok(s) => s,
                Err(_) => return Err(CpuError::DeviceError(value)),
            }
        );
        Ok(())
    }
}

impl Device for TerminalIn {
    fn get_address_space(&self) -> RangeInclusive<u64> {
        todo!()
    }

    fn load(&self, address: u64) -> Result<u64, CpuError> {
        todo!()
    }

    fn push(&self, address: u64, value: u64) -> Result<(), CpuError> {
        todo!()
    }
}
