use std::num::NonZero;

use crate::parsers::numeric::little_endian::LittleEndian;

pub mod i16;
pub mod i32;
pub mod i64;
pub mod u16;
pub mod u32;
pub mod u64;

pub type LittleEndianNonZero<T> = LittleEndian<NonZero<T>>;
