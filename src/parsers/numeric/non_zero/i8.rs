use core::num::NonZeroI8;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::i8::I8Parser,
    result::ParseResult,
};

pub struct NonZeroI8Parser;

impl ParsableDefinition<'_> for NonZeroI8Parser {
    type Output = NonZeroI8;

    type Error = ExpectedNumberParseError;

    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::NonZeroI8)
    }
}

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for NonZeroI8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = I8Parser.parse(parser)?;

        let Some(value) = NonZeroI8::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                ExpectedNumberParseError(NumericExpectation::NonZeroI8),
            );
        };

        Ok(value)
    }
}

pub struct OptionalNonZeroI8Parser;

impl ParsableDefinition<'_> for OptionalNonZeroI8Parser {
    type Output = Option<NonZeroI8>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::I8)
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalNonZeroI8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = I8Parser.parse(parser)?;

        Ok(NonZeroI8::new(value))
    }
}
