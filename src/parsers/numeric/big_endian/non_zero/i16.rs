use core::num::NonZeroI16;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::i16::BigEndianI16Parser,
    result::ParseResult,
};

pub struct BigEndianNonZeroI16Parser;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianNonZeroI16Parser {
    type Output = NonZeroI16;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI16Parser.parse(parser)?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::BigEndianNonZeroI16),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianNonZeroI16Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::BigEndianNonZeroI16)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<BigEndianNonZeroI16Parser> {
    type Output = Option<NonZeroI16>;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianI16Parser.parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<BigEndianNonZeroI16Parser> {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::BigEndianI16)
    }
}
