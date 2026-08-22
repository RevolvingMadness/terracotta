use crate::input::token::Token;

impl Token for u8 {
    #[inline]
    fn len(&self) -> usize {
        size_of::<Self>()
    }
}
