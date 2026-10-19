use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct BigEndianU64Parser;

impl<E> Parsable<'_, [u8], E> for BigEndianU64Parser {
    type Output = u64;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u64::from_be_bytes(bytes))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianU64Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::BigEndianU64)
    }
}
