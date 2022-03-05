use crate::{arch::Arch, errors::CpuError, Cpu, Device};

impl<T: Arch> Cpu<T> {
    pub fn load(&self, arg: T) -> Result<T, CpuError<T>> {
        match arg {
            _ if arg == self.REG_ZERO => Ok(self.reg_zero),
            _ if (self.DATA_START..=self.DATA_END).contains(&arg) => {
                Ok(self.data_cache[(arg - self.DATA_START).as_usize()])
            }
            _ if (self.INSTRUCTION_START..=self.INSTRUCTION_END).contains(&arg) => {
                Ok(self.instruction_cache.get(arg - self.INSTRUCTION_START))
            }
            _ => {
                match self
                    .devices
                    .iter()
                    .filter_map(|dev| match dev.get_address_space().contains(&arg) {
                        true => Some(dev),
                        false => None,
                    })
                    .collect::<Vec<&Box<dyn Device<T>>>>()
                    .get(0)
                {
                    Some(dev) => return dev.load(arg),
                    None => return Err(CpuError::AddressNotPopulated(arg)),
                }
            }
        }
    }

    pub fn push(&mut self, arg1: T, arg2: T) -> Result<(), CpuError<T>> {
        //dbg!(arg1, arg2);
        //dbg!(self.reg_zero);
        match arg1 {
            _ if self.REG_ZERO == arg1 => {
                self.reg_zero = arg2;
                Ok(())
            }
            _ if (self.DATA_START..=self.DATA_END).contains(&arg1) => {
                
                self.data_cache[(arg1 - self.DATA_START).as_usize()] = arg2;
                Ok(())
            }
            _ if (self.INSTRUCTION_START..=self.INSTRUCTION_END).contains(&arg1) => {
                self.instruction_cache
                    .set(arg1 - self.INSTRUCTION_START, arg2);
                Ok(())
            }
            _ => {
                match self
                    .devices
                    .iter()
                    .filter_map(|dev| match dev.get_address_space().contains(&arg1) {
                        true => Some(dev),
                        false => None,
                    })
                    .collect::<Vec<&Box<dyn Device<T>>>>()
                    .get(0)
                {
                    Some(dev) => return dev.push(arg1, arg2),
                    None => return Err(CpuError::AddressNotPopulated(arg1)),
                }
            }
        }
    }
}
