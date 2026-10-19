use core::marker::PhantomData;

use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableDefinition},
    parser::Parser,
    result::ParseResult,
    span::Span,
};

pub struct SpannedParser<'input, I: Input + ?Sized + 'input, P: Parsable<'input, I>> {
    _phantom_data: PhantomData<&'input I>,
    parsable: P,
}

impl<'input, I: Input + ?Sized + 'input, P: Parsable<'input, I>> ParsableDefinition<'input>
    for SpannedParser<'input, I, P>
{
    type Output = Spanned<P::Output>;

    type Error = P::Error;

    fn error(&self) -> Self::Error {
        self.parsable.error()
    }
}

impl<'input, I: Input + ?Sized + 'input, P: Parsable<'input, I>> Parsable<'input, I>
    for SpannedParser<'input, I, P>
{
    fn parse(&self, parser: &mut Parser<'input, I, Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = self.parsable.parse(parser)?;

        let end = parser.position();

        Ok(Spanned {
            span: start.span(end),
            value,
        })
    }
}

pub trait SpannedParserExt<'input, I: Input + ?Sized + 'input>:
    Parsable<'input, I> + Sized
{
    #[inline]
    #[must_use]
    fn parse_spanned(self) -> SpannedParser<'input, I, Self> {
        SpannedParser {
            _phantom_data: PhantomData,
            parsable: self,
        }
    }
}

impl<'input, I: Input + ?Sized + 'input, P: Parsable<'input, I>> SpannedParserExt<'input, I> for P {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Spanned<T: ?Sized> {
    pub span: Span,
    pub value: T,
}

impl<T> Spanned<T> {
    #[inline]
    #[must_use]
    pub const fn new(span: Span, value: T) -> Self {
        Self { span, value }
    }
}

pub trait SpannedExt {
    #[must_use]
    fn with_span(self, span: Span) -> Spanned<Self>;
}

impl<T> SpannedExt for T {
    #[inline]
    fn with_span(self, span: Span) -> Spanned<Self> {
        Spanned::new(span, self)
    }
}
