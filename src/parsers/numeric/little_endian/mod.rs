use std::marker::PhantomData;

pub mod f32;
pub mod f64;
pub mod i16;
pub mod i32;
pub mod i64;
pub mod non_zero;
pub mod u16;
pub mod u32;
pub mod u64;

pub struct LittleEndian<T>(PhantomData<T>);
