use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::LittleEndian,
    result::ParseResult,
};

pub type LittleEndianI32 = LittleEndian<i32>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianI32 {
    type Output = i32;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianI32)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i32::from_le_bytes(bytes))
    }
}
