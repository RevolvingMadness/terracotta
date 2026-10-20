use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianF64Parser;

impl ParsableDefinition<'_> for BigEndianF64Parser {
    type Output = f64;

    type Error = ExpectedNumberParseError;

    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianF64)
    }
}

impl<E> Parsable<'_, [u8], E> for BigEndianF64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_be_bytes(bytes))
    }
}
