use core::num::NonZeroU8;

use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::u8::U8Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct NonZeroU8Parser;

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for NonZeroU8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = U8Parser.parse(parser)?;

        let Some(value) = NonZeroU8::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                ExpectedNumberParseError(NumericExpectation::NonZeroU8),
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for NonZeroU8Parser {
    type Output = NonZeroU8;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::NonZeroU8)
    }
}

pub struct OptionalNonZeroU8Parser;

impl<E> Parsable<'_, [u8], E> for OptionalNonZeroU8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = U8Parser.parse(parser)?;

        Ok(NonZeroU8::new(value))
    }
}

impl ParsableDefinition<'_> for OptionalNonZeroU8Parser {
    type Output = Option<NonZeroU8>;

    type Error = ExpectedNumberParseError;

    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::U8)
    }
}
