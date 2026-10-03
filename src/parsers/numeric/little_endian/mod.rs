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

pub struct LittleEndian<T>(PhantomData<T>);

impl<T> Debug for LittleEndian<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("LittleEndian").finish()
    }
}

impl<T> Clone for LittleEndian<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for LittleEndian<T> {}

impl<T> Default for LittleEndian<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> PartialEq for LittleEndian<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for LittleEndian<T> {}

impl<T> PartialOrd for LittleEndian<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for LittleEndian<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for LittleEndian<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
