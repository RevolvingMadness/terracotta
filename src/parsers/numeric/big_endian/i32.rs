use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    result::ParseResult,
};

pub struct BigEndianI32Parser;

impl ParsableDefinition<'_> for BigEndianI32Parser {
    type Output = i32;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianI32
    }
}

impl Parsable<'_, [u8]> for BigEndianI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i32::from_be_bytes(bytes))
    }
}
