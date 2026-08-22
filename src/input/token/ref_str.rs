use crate::input::token::Token;

impl Token for &str {
    #[inline]
    fn len(&self) -> usize {
        str::len(self)
    }
}
