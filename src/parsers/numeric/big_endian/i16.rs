use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct BigEndianI16Parser;

impl ParsableDefinition<'_> for BigEndianI16Parser {
    type Output = i16;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianI16)
    }
}

impl<E> Parsable<'_, [u8], E> for BigEndianI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2)?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i16::from_be_bytes(bytes))
    }
}
