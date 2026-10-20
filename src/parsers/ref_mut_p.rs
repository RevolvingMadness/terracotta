use crate::{
    input::Input,
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

impl<'input, P: ParsableDefinition<'input> + ?Sized> ParsableDefinition<'input> for &mut P {
    type Output = P::Output;

    type Error = P::Error;

    #[inline]
    fn error(&self) -> Self::Error {
        (**self).error()
    }
}

impl<'input, I, E, P> Parsable<'input, I, E> for &mut P
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E> + ?Sized,
{
    #[inline]
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        (**self).parse(parser)
    }
}
