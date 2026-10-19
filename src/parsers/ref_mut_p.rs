use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableDefinition},
    parser::Parser,
    result::ParseResult,
};

impl<'input, P: ParsableDefinition<'input> + ?Sized> ParsableDefinition<'input> for &mut P {
    type Output = P::Output;

    type Error = P::Error;

    #[inline]
    fn error(&self) -> Self::Error {
        (**self).error()
    }
}

impl<'input, I: Input + ?Sized + 'input, P: Parsable<'input, I> + ?Sized> Parsable<'input, I>
    for &mut P
{
    #[inline]
    fn parse(&self, parser: &mut Parser<'input, I, P::Error>) -> ParseResult<Self::Output> {
        (**self).parse(parser)
    }
}
