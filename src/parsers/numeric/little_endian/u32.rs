use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    result::ParseResult,
};

pub struct LittleEndianU32Parser;

impl Parsable<'_, [u8]> for LittleEndianU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u32::from_le_bytes(bytes))
    }
}

impl ParsableDefinition<'_> for LittleEndianU32Parser {
    type Output = u32;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianU32
    }
}
