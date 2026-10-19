use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct LittleEndianU32Parser;

impl<E> Parsable<'_, [u8], E> for LittleEndianU32Parser {
    type Output = u32;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u32::from_le_bytes(bytes))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianU32Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianU32)
    }
}
