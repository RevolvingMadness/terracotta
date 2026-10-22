use crate::{
    input::Input,
    parser::Parser,
    results::ParseResult,
    traits::{
        parsable::Parsable, parsable_definition::ParsableDefinition,
        parsable_with_context::ParsableWithContext,
    },
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ParsableWithAnyContext<P> {
    parsable: P,
}

impl<P> ParsableDefinition for ParsableWithAnyContext<P>
where
    P: ParsableDefinition,
{
    type Output<'input, I: Input + ?Sized + 'input> = P::Output<'input, I>;

    type Error = P::Error;

    #[inline]
    fn error(&self) -> Self::Error {
        self.parsable.error()
    }
}

impl<'input, I, E, P> Parsable<'input, I, E> for ParsableWithAnyContext<P>
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    #[inline]
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output<'input, I>> {
        self.parsable.parse(parser)
    }
}

impl<'input, I, E, P, Context> ParsableWithContext<'input, I, E, Context>
    for ParsableWithAnyContext<P>
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    #[inline]
    fn parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        _: &mut Context,
    ) -> ParseResult<Self::Output<'input, I>> {
        self.parsable.parse(parser)
    }
}

impl<P> ParsableWithAnyContext<P> {
    #[inline]
    #[must_use]
    pub const fn new(parsable: P) -> Self {
        Self { parsable }
    }
}

pub trait ParsableWithAnyContextParsableExt<'input, I, E, P>:
    Parsable<'input, I, E> + Sized
where
    I: Input + ?Sized + 'input,
{
    #[inline]
    #[must_use]
    fn parsable_with_any_context(self) -> ParsableWithAnyContext<Self> {
        ParsableWithAnyContext::new(self)
    }
}

impl<'input, I, E, P> ParsableWithAnyContextParsableExt<'input, I, E, P> for P
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
}
