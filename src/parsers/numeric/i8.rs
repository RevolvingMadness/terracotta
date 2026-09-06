use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for i8 {
    type Output = Self;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::I8)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = parser.advance()?;

        Ok(value as Self)
    }
}
