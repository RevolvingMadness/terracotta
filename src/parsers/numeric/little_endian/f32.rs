use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

pub struct LittleEndianF32Parser;

impl<E> Parsable<'_, [u8], E> for LittleEndianF32Parser {
    type Output = f32;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f32::from_le_bytes(bytes))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianF32Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianF32)
    }
}
