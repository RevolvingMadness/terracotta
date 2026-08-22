pub trait Token {
    #[must_use]
    fn len(&self) -> usize;

    #[inline]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Token for char {
    #[inline]
    fn len(&self) -> usize {
        self.len_utf8()
    }
}

impl Token for &str {
    #[inline]
    fn len(&self) -> usize {
        str::len(self)
    }
}
