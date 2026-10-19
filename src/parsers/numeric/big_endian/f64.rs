use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct BigEndianF64Parser;

impl<E> Parsable<'_, [u8], E> for BigEndianF64Parser {
    type Output = f64;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_be_bytes(bytes))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianF64Parser {
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::BigEndianF64)
    }
}
