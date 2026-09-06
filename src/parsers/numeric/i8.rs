use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

impl<E> Parsable<'_, [u8], E> for i8 {
    type Output = Self;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = parser.advance()?;

        Ok(value as Self)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for i8 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::I8)
    }
}
