use crate::{input::Input, parsable_traits::Parsable, parser::Parser, result::ParseResult};

impl<'input, I: Input + ?Sized, E, P: Parsable<'input, I, E> + ?Sized> Parsable<'input, I, E>
    for &mut P
{
    type Output = P::Output;

    #[inline]
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        (**self).parse(parser)
    }
}
