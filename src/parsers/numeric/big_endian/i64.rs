use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianI64Parser;

impl ParsableDefinition<'_> for BigEndianI64Parser {
    type Output = i64;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianI64)
    }
}

impl<E> Parsable<'_, [u8], E> for BigEndianI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i64::from_be_bytes(bytes))
    }
}
