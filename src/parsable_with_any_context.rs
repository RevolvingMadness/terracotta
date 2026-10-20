use core::marker::PhantomData;

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
pub struct ParsableWithAnyContext<'input, I: Input + ?Sized + 'input, E, P: Parsable<'input, I, E>>
{
    _phantom_data: PhantomData<(&'input I, E)>,
    parsable: P,
}

impl<'input, I, E, P> ParsableDefinition<'input> for ParsableWithAnyContext<'input, I, E, P>
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    type Output = P::Output;

    type Error = P::Error;

    #[inline]
    fn error(&self) -> Self::Error {
        self.parsable.error()
    }
}

impl<'input, I, E, P> Parsable<'input, I, E> for ParsableWithAnyContext<'input, I, E, P>
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    #[inline]
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        self.parsable.parse(parser)
    }
}

impl<'input, I, E, P, Context> ParsableWithContext<'input, I, E, Context>
    for ParsableWithAnyContext<'input, I, E, P>
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    #[inline]
    fn parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        _: &mut Context,
    ) -> ParseResult<Self::Output> {
        self.parsable.parse(parser)
    }
}

impl<'input, I: Input + ?Sized + 'input, E, P: Parsable<'input, I, E>>
    ParsableWithAnyContext<'input, I, E, P>
{
    #[inline]
    #[must_use]
    pub const fn new(parsable: P) -> Self {
        Self {
            _phantom_data: PhantomData,
            parsable,
        }
    }
}

pub trait ParsableWithAnyContextParsableExt<'input, I, E, P>
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    #[must_use]
    fn parsable_with_any_context(self) -> ParsableWithAnyContext<'input, I, E, P>;
}

impl<'input, I, E, P> ParsableWithAnyContextParsableExt<'input, I, E, P> for P
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    #[inline]
    fn parsable_with_any_context(self) -> ParsableWithAnyContext<'input, I, E, P> {
        ParsableWithAnyContext::new(self)
    }
}
