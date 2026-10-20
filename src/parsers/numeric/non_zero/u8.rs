use core::num::NonZeroU8;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::u8::U8Parser,
    result::ParseResult,
};

pub struct NonZeroU8Parser;

impl Parsable<'_, [u8]> for NonZeroU8Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = U8Parser.parse(parser)?;

        let Some(value) = NonZeroU8::new(value) else {
            let end = parser.position();

            return parser
                .add_error_with_span_result(start.span(end), NumericExpectation::NonZeroU8);
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for NonZeroU8Parser {
    type Output = NonZeroU8;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::NonZeroU8
    }
}

pub struct OptionalNonZeroU8Parser;

impl Parsable<'_, [u8]> for OptionalNonZeroU8Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = U8Parser.parse(parser)?;

        Ok(NonZeroU8::new(value))
    }
}

impl ParsableDefinition<'_> for OptionalNonZeroU8Parser {
    type Output = Option<NonZeroU8>;

    type Error = NumericExpectation;

    fn error(&self) -> Self::Error {
        NumericExpectation::U8
    }
}
