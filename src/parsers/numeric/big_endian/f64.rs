use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::ParseResult,
};

pub type BigEndianF64 = BigEndian<f64>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianF64 {
    type Output = f64;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianF64)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_be_bytes(bytes))
    }
}
