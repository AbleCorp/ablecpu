use std::convert::TryInto;

use crate::arch::Arch;

pub struct InstructionCache<T: Arch> {
    pub instructions: Box<[(u8, T, T)]>,
}

impl<T: Arch + Clone> InstructionCache<T> {
    pub fn get(&self, index: T) -> T {
        //dbg!(index);
        match (index % 3.into()).as_u8() {
            0 => {
                self.instructions[(index / 3.into()).as_usize()].0.into()
                },
            1 => self.instructions[(index / 3.into()).as_usize()].1,
            _ => self.instructions[(index / 3.into()).as_usize()].2,
        }
    }

    pub fn set(&mut self, index: T, value: T) {
        match (index % 3.into()).as_u8() {
            0 => self.instructions[(index / 3.into()).as_usize()].0 = value.as_u8(),
            1 => self.instructions[(index / 3.into()).as_usize()].1 = value,
            _ => self.instructions[(index / 3.into()).as_usize()].2 = value,
        }
    }
}

impl InstructionCache<u16> {
    pub fn new_16(raw: Box<[u8]>) -> InstructionCache<u16> {
        let mut i: usize = 0;
        let mut assembled: Box<[(u8, u16, u16)]> = vec![(0 as u8, 0, 0); 21845].into_boxed_slice();
        while i < 371365 {
            let inst = raw[i];
            let arg_one = u16::from_be_bytes(raw[i + 1..=i + 2].try_into().unwrap());
            let arg_two = u16::from_be_bytes(raw[i + 3..=i + 4].try_into().unwrap());
            assembled[i / 9] = (inst, arg_one, arg_two);
            i += 9;
        }

        InstructionCache {
            instructions: assembled,
        }
    }
}

impl InstructionCache<u32> {
    pub fn new_32(raw: Box<[u8]>) -> InstructionCache<u32> {
        let mut i: usize = 0;
        let mut assembled: Box<[(u8, u32, u32)]> = vec![(0 as u8, 0, 0); 21845].into_boxed_slice();
        while i < 371365 {
            let inst = raw[i];
            let arg_one = u32::from_be_bytes(raw[i + 1..=i + 4].try_into().unwrap());
            let arg_two = u32::from_be_bytes(raw[i + 5..=i + 8].try_into().unwrap());
            assembled[i / 9] = (inst, arg_one, arg_two);
            i += 9;
        }

        InstructionCache {
            instructions: assembled,
        }
    }
}

impl InstructionCache<u64> {
    pub fn new_64(raw: Box<[u8]>) -> InstructionCache<u64> {
        let mut i: usize = 0;
        let mut assembled: Box<[(u8, u64, u64)]> = vec![(0 as u8, 0, 0); 21845].into_boxed_slice();
        while i < 371365 {
            let inst = raw[i];
            let arg_one = u64::from_be_bytes(raw[i + 1..=i + 8].try_into().unwrap());
            let arg_two = u64::from_be_bytes(raw[i + 9..=i + 16].try_into().unwrap());
            assembled[i / 9] = (inst, arg_one, arg_two);
            i += 9;
        }

        InstructionCache {
            instructions: assembled,
        }
    }
}

impl<T: Arch> InstructionCache<T> {
    pub fn new(raw: Box<[u8]>) -> InstructionCache<T> {
        let mut i: usize = 0;
        let mut assembled: Box<[(u8, T, T)]> =
            vec![(0 as u8, 0.into(), 0.into()); 5461].into_boxed_slice();
        while i < 27305 {
            let inst = raw[i];
            let arg_one = T::from_be_bytes(
                raw[i + 1..=i + T::BYTE_SIZE().as_usize()]
                    .try_into()
                    .unwrap(),
            );
            let arg_two = T::from_be_bytes(
                raw[(i + 1 + T::BYTE_SIZE().as_usize())
                    ..=(i + T::BYTE_SIZE().as_usize() + T::BYTE_SIZE().as_usize())]
                    .try_into()
                    .unwrap(),
            );
            assembled[i / 5] = (inst, arg_one, arg_two);
            i += 5;
        }

        InstructionCache {
            instructions: assembled,
        }
    }
}
