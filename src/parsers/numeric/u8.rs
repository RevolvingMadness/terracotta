use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

impl<'input, E: ExpectedNumericParseError> Parsable<'input, [Self], E> for u8 {
    type Output = Self;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::U8)
    }

    fn parse(parser: &mut Parser<'input, [Self], E>) -> ParseResult<Self::Output> {
        let value = parser.advance()?;

        Ok(value)
    }
}
