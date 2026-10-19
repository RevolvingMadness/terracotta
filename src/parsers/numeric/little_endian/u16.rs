use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    result::ParseResult,
};

pub struct LittleEndianU16Parser;

impl ParsableDefinition<'_> for LittleEndianU16Parser {
    type Output = u16;

    type Error = NumericExpectation;

    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianU16
    }
}

impl Parsable<'_, [u8]> for LittleEndianU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2)?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u16::from_le_bytes(bytes))
    }
}
