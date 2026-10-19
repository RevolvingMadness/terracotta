pub mod char;
pub mod ref_str;
pub mod u8;

pub trait Token: PartialEq + Clone {
    #[must_use]
    fn len(&self) -> usize;

    #[inline]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
