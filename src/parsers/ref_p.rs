use crate::{input::Input, parsable_traits::ParsableInstance, parser::Parser, result::ParseResult};

impl<'input, I: Input + ?Sized, E, P: ParsableInstance<'input, I, E> + ?Sized>
    ParsableInstance<'input, I, E> for &P
{
    type Output = P::Output;

    #[inline]
    fn instance_parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        (*self).instance_parse(parser)
    }
}
