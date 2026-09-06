use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::ParseResult,
};

pub type BigEndianU32 = BigEndian<u32>;

impl<E> Parsable<'_, [u8], E> for BigEndianU32 {
    type Output = u32;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u32::from_be_bytes(bytes))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianU32 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianU32)
    }
}
