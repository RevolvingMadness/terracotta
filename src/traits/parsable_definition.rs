use crate::input::Input;

pub trait ParsableDefinition {
    type Output<'input, I: Input + ?Sized + 'input>;

    type Error;

    #[must_use]
    fn error(&self) -> Self::Error;
}
