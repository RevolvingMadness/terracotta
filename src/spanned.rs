use core::marker::PhantomData;

use crate::{
    input::Input,
    parser::Parser,
    results::ParseResult,
    span::Span,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct SpannedParser<'input, I: Input + ?Sized + 'input, E, P: Parsable<'input, I, E>> {
    _phantom_data: PhantomData<(&'input I, E)>,
    parsable: P,
}

impl<'input, I, E, P> ParsableDefinition<'input> for SpannedParser<'input, I, E, P>
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    type Output = Spanned<P::Output>;

    type Error = P::Error;

    fn error(&self) -> Self::Error {
        self.parsable.error()
    }
}

impl<'input, I, E, P> Parsable<'input, I, E> for SpannedParser<'input, I, E, P>
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = self.parsable.parse(parser)?;

        let end = parser.position();

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
    fn parse_spanned(self) -> SpannedParser<'input, I, E, Self> {
        SpannedParser {
            _phantom_data: PhantomData,
            parsable: self,
        }
    }
}

impl<'input, I, E, P> SpannedParserExt<'input, I, E> for P
where
    I: Input + ?Sized + 'input,
    P: Parsable<'input, I, E>,
{
}

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
