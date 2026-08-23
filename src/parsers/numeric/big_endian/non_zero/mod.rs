use std::num::NonZero;

use crate::parsers::numeric::big_endian::BigEndian;

pub mod i16;
pub mod i32;
pub mod i64;
pub mod u16;
pub mod u32;
pub mod u64;

pub type BigEndianNonZero<T> = BigEndian<NonZero<T>>;
