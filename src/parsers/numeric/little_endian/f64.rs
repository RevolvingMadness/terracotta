use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::LittleEndian,
    result::ParseResult,
};

pub type LittleEndianF64 = LittleEndian<f64>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianF64 {
    type Output = f64;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianF64)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_le_bytes(bytes))
    }
}
