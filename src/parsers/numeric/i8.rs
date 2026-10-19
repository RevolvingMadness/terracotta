use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct I8Parser;

impl<E> Parsable<'_, [u8], E> for I8Parser {
    type Output = i8;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = parser.advance_result()?;

        Ok(value as i8)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for I8Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::I8)
    }
}
