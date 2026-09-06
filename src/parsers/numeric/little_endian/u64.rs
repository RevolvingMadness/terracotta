use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::LittleEndian,
    result::ParseResult,
};

pub type LittleEndianU64 = LittleEndian<u64>;

impl<E> Parsable<'_, [u8], E> for LittleEndianU64 {
    type Output = u64;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u64::from_le_bytes(bytes))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianU64 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianU64)
    }
}
