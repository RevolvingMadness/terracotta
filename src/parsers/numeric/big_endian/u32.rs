use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct BigEndianU32Parser;

impl<E> Parsable<'_, [u8], E> for BigEndianU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u32::from_be_bytes(bytes))
    }
}

impl ParsableDefinition<'_> for BigEndianU32Parser {
    type Output = u32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianU32)
    }
}
