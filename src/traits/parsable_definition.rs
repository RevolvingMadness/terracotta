pub trait ParsableDefinition<'input> {
    type Output;

    type Error;

    #[must_use]
    fn error(&self) -> Self::Error;
}
