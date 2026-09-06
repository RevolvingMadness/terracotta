use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::LittleEndian,
    result::ParseResult,
};

pub type LittleEndianI64 = LittleEndian<i64>;

impl<E> Parsable<'_, [u8], E> for LittleEndianI64 {
    type Output = i64;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i64::from_le_bytes(bytes))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianI64 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianI64)
    }
}
