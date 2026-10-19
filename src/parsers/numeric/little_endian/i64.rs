use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    result::ParseResult,
};

pub struct LittleEndianI64Parser;

impl Parsable<'_, [u8]> for LittleEndianI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i64::from_le_bytes(bytes))
    }
}

impl ParsableDefinition<'_> for LittleEndianI64Parser {
    type Output = i64;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianI64
    }
}
