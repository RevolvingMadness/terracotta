use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::ParseResult,
};

pub type BigEndianU64 = BigEndian<u64>;

impl<E> Parsable<'_, [u8], E> for BigEndianU64 {
    type Output = u64;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u64::from_be_bytes(bytes))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianU64 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianU64)
    }
}
