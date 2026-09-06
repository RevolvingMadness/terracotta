use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::LittleEndian,
    result::ParseResult,
};

pub type LittleEndianF32 = LittleEndian<f32>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianF32 {
    type Output = f32;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianF32)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f32::from_le_bytes(bytes))
    }
}
