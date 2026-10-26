#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaybeExpectedEndOfInputParseError<E> {
    ExpectedEndOfInput,
    Error(E),
}

impl<E> From<E> for MaybeExpectedEndOfInputParseError<E> {
    fn from(error: E) -> Self {
        Self::Error(error)
    }
}
