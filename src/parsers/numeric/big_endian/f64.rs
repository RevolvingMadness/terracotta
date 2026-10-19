use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    result::ParseResult,
};

pub struct BigEndianF64Parser;

impl ParsableDefinition<'_> for BigEndianF64Parser {
    type Output = f64;

    type Error = NumericExpectation;

    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianF64
    }
}

impl Parsable<'_, [u8]> for BigEndianF64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_be_bytes(bytes))
    }
}
