use crate::input::token::Token;

impl Token for char {
    #[inline]
    fn len(&self) -> usize {
        self.len_utf8()
    }
}
