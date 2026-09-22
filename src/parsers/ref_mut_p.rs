use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableInstance},
    parser::Parser,
    result::ParseResult,
};

impl<'input, I: Input + ?Sized, E, P: Parsable<'input, I, E> + ?Sized> Parsable<'input, I, E>
    for &mut P
{
    type Output = P::Output;

    #[inline]
    fn parse(parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        P::parse(parser)
    }
}

impl<'input, I: Input + ?Sized, E, P: ParsableInstance<'input, I, E> + ?Sized>
    ParsableInstance<'input, I, E> for &mut P
{
    type Output = P::Output;

    #[inline]
    fn instance_parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        (**self).instance_parse(parser)
    }
}
