use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct LittleEndianI16Parser;

impl<E> Parsable<'_, [u8], E> for LittleEndianI16Parser {
    type Output = i16;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2)?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i16::from_le_bytes(bytes))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianI16Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianI16)
    }
}
