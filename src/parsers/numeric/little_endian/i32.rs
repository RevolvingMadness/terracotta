use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianI32Parser;

impl<E> Parsable<'_, [u8], E> for LittleEndianI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i32::from_le_bytes(bytes))
    }
}

impl ParsableDefinition<'_> for LittleEndianI32Parser {
    type Output = i32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianI32)
    }
}
