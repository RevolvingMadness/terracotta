use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::ParseResult,
};

pub type BigEndianU16 = BigEndian<u16>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianU16 {
    type Output = u16;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianU16)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2)?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u16::from_be_bytes(bytes))
    }
}
