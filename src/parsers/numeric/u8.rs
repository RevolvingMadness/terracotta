use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

impl<E> Parsable<'_, [Self], E> for u8 {
    type Output = Self;

    fn parse(parser: &mut Parser<[Self], E>) -> ParseResult<Self::Output> {
        let value = parser.advance()?;

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for u8 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::U8)
    }
}
