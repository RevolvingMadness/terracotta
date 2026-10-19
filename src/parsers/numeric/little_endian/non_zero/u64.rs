use core::num::NonZeroU64;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::u64::LittleEndianU64Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroU64Parser;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroU64Parser {
    type Output = NonZeroU64;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU64Parser.parse(parser)?;

        let Some(value) = NonZeroU64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::LittleEndianNonZeroU64),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianNonZeroU64Parser {
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroU64)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroU64Parser> {
    type Output = Option<NonZeroU64>;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianU64Parser.parse(parser)?;

        Ok(NonZeroU64::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<LittleEndianNonZeroU64Parser> {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianU64)
    }
}
