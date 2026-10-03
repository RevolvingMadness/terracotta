use core::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub mod f32;
pub mod f64;
pub mod i16;
pub mod i32;
pub mod i64;
pub mod non_zero;
pub mod u16;
pub mod u32;
pub mod u64;

pub struct BigEndian<T>(PhantomData<T>);

impl<T> Debug for BigEndian<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BigEndian").finish()
    }
}

impl<T> Clone for BigEndian<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for BigEndian<T> {}

impl<T> Default for BigEndian<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> PartialEq for BigEndian<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for BigEndian<T> {}

impl<T> PartialOrd for BigEndian<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for BigEndian<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for BigEndian<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
