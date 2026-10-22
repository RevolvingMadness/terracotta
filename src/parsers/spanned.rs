use crate::{
    input::Input,
    parser::Parser,
    results::ParseResult,
    spanned::Spanned,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SpannedParser<P> {
    parsable: P,
}

impl<'input, P> ParsableDefinition<'input> for SpannedParser<P>
where
    P: ParsableDefinition<'input>,
{
    type Output = Spanned<P::Output>;

    type Error = P::Error;

    fn error(&self) -> Self::Error {
        self.parsable.error()
    }
}

impl<'input, I, E, P> Parsable<'input, I, E> for SpannedParser<P>
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        let start = parser.position;

        let value = self.parsable.parse(parser)?;

        let end = parser.position;

        Ok(Spanned {
            span: start.span(end),
            value,
        })
    }
}

pub trait SpannedParserExt<'input, I, E>: Parsable<'input, I, E> + Sized
where
    I: Input + ?Sized + 'input,
{
    #[inline]
    #[must_use]
    fn parse_spanned(self) -> SpannedParser<Self> {
        SpannedParser { parsable: self }
    }
}

impl<'input, I, E, P> SpannedParserExt<'input, I, E> for P
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
}
