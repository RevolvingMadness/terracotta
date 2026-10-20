use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct LittleEndianU16Parser;

impl ParsableDefinition<'_> for LittleEndianU16Parser {
    type Output = u16;

    type Error = ExpectedNumberParseError;

    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianU16)
    }
}

impl<E> Parsable<'_, [u8], E> for LittleEndianU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2)?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u16::from_le_bytes(bytes))
    }
}
