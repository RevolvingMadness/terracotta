pub trait ParsableDefinition {
    type Output<'input>;

    type Error;

    #[must_use]
    fn error(&self) -> Self::Error;
}
