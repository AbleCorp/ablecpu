use std::ops::RangeInclusive;

use ablecpu_vm::{errors::Cpu64Error, Device};

pub(crate) struct TerminalOut {}

pub(crate) struct TerminalIn {}

impl Device for TerminalOut {
    fn get_address_space(&self) -> RangeInclusive<u64> {
        131073..=131073
    }

    fn load(&self, _address: u64) -> Result<u64, Cpu64Error> {
        println!("{}", _address);
        Ok(0)
    }

    fn push(&self, _address: u64, value: u64) -> Result<(), Cpu64Error> {
        print!(
            "{}",
            match std::str::from_utf8(&value.to_be_bytes()) {
                Ok(s) => s,
                Err(_) => return Err(Cpu64Error::DeviceError(value)),
            }
        );
        Ok(())
    }
}

impl Device for TerminalIn {
    fn get_address_space(&self) -> RangeInclusive<u64> {
        todo!()
    }

    fn load(&self, address: u64) -> Result<u64, Cpu64Error> {
        todo!()
    }

    fn push(&self, address: u64, value: u64) -> Result<(), Cpu64Error> {
        todo!()
    }
}
