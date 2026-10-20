use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianU64Parser;

impl<E> Parsable<'_, [u8], E> for LittleEndianU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u64::from_le_bytes(bytes))
    }
}

impl ParsableDefinition<'_> for LittleEndianU64Parser {
    type Output = u64;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianU64)
    }
}
