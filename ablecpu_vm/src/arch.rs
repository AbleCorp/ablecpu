use std::{
    convert::TryInto,
    ops::{Add, Div, Mul, Rem, Sub},
};

pub trait Arch:
    From<u8>
    + std::ops::AddAssign
    + Add<Output = Self>
    + std::ops::SubAssign
    + Sub<Output = Self>
    + std::ops::MulAssign
    + Mul<Output = Self>
    + std::ops::DivAssign
    + Div<Output = Self>
    + std::ops::RemAssign
    + Rem<Output = Self>
    + Clone
    + Copy
    + Ord
{
    fn from_be_bytes(b: &[u8]) -> Self;
    fn as_usize(&self) -> usize;
    fn as_u8(&self) -> u8;
    fn from_i32(i: i32) -> Self;
}

impl Arch for u64 {
    fn from_be_bytes(b: &[u8]) -> Self {
        u64::from_be_bytes(b.try_into().unwrap())
    }

    fn as_usize(&self) -> usize {
        *self as usize
    }

    fn as_u8(&self) -> u8 {
        *self as u8
    }

    fn from_i32(i: i32) -> Self {
        i as u64
    }
}
impl Arch for u32 {
    fn from_be_bytes(b: &[u8]) -> Self {
        u32::from_be_bytes(b.try_into().unwrap())
    }

    fn as_usize(&self) -> usize {
        *self as usize
    }

    fn as_u8(&self) -> u8 {
        *self as u8
    }

    fn from_i32(i: i32) -> Self {
        i as u32
    }
}
impl Arch for u16 {
    fn from_be_bytes(b: &[u8]) -> Self {
        u16::from_be_bytes(b.try_into().unwrap())
    }

    fn as_usize(&self) -> usize {
        *self as usize
    }

    fn as_u8(&self) -> u8 {
        *self as u8
    }

    fn from_i32(i: i32) -> Self {
        i as u16
    }
}
