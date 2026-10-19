use core::num::NonZeroI32;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::i32::LittleEndianI32Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroI32Parser;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroI32Parser {
    type Output = NonZeroI32;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI32Parser.parse(parser)?;

        let Some(value) = NonZeroI32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::LittleEndianNonZeroI32),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianNonZeroI32Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroI32)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroI32Parser> {
    type Output = Option<NonZeroI32>;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianI32Parser.parse(parser)?;

        Ok(NonZeroI32::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<LittleEndianNonZeroI32Parser> {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianI32)
    }
}
