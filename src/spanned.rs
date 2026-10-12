use core::marker::PhantomData;

use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableInstance},
    parser::Parser,
    result::ParseResult,
    span::Span,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Spanned<T: ?Sized> {
    pub span: Span,
    pub value: T,
}

impl<'input, I: Input + ?Sized, E, P: Parsable<'input, I, E>> Parsable<'input, I, E>
    for Spanned<P>
{
    type Output = Spanned<P::Output>;

    fn parse(parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = P::parse(parser)?;

        let end = parser.position();

        Ok(Spanned {
            span: start.span(end),
            value,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpannedParsableInstance<'input, I: Input + ?Sized, E, P: ParsableInstance<'input, I, E>>
{
    _phantom_data: PhantomData<(&'input I, E)>,
    pub parsable_instance: P,
}

impl<'input, I: Input + ?Sized, E, P: ParsableInstance<'input, I, E>> ParsableInstance<'input, I, E>
    for SpannedParsableInstance<'input, I, E, P>
{
    type Output = Spanned<P::Output>;

    fn instance_parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let output = self.parsable_instance.instance_parse(parser)?;

        let end = parser.position();

        Ok(Spanned {
            span: start.span(end),
            value: output,
        })
    }
}

pub trait SpannedParserExt<'input, I: Input + ?Sized, E>:
    ParsableInstance<'input, I, E> + Sized
{
    #[inline]
    #[must_use]
    fn parse_spanned(self) -> SpannedParsableInstance<'input, I, E, Self> {
        SpannedParsableInstance {
            _phantom_data: PhantomData,
            parsable_instance: self,
        }
    }
}

impl<'input, I: Input + ?Sized, E, P: ParsableInstance<'input, I, E>> SpannedParserExt<'input, I, E>
    for P
{
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

impl<T> Spanned<T> {
    #[inline]
    #[must_use]
    pub const fn new(span: Span, value: T) -> Self {
        Self { span, value }
    }
}
