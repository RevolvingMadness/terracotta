use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    result::ParseResult,
};

pub struct LittleEndianU64Parser;

impl Parsable<'_, [u8]> for LittleEndianU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u64::from_le_bytes(bytes))
    }
}

impl ParsableDefinition<'_> for LittleEndianU64Parser {
    type Output = u64;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianU64
    }
}
