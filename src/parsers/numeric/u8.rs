use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct U8Parser;

impl<E> Parsable<'_, [u8], E> for U8Parser {
    type Output = u8;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = parser.advance_result()?;

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for U8Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::U8)
    }
}
