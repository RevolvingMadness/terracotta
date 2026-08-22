pub mod char;
pub mod ref_str;
pub mod u8;

pub trait Token: PartialEq {
    #[must_use]
    fn len(&self) -> usize;

    #[inline]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: ?Sized + Token> Token for &T {
    fn len(&self) -> usize {
        T::len(self)
    }
}

impl<T: ?Sized + Token> Token for &mut T {
    fn len(&self) -> usize {
        T::len(self)
    }
}
