use crate::{
    parsable_traits::Parsable, parser::Parser, parsers::numeric::big_endian::BigEndian,
    result::ParseResult,
};

pub type BigEndianU64 = BigEndian<u64>;

impl Parsable<'_, [u8]> for BigEndianU64 {
    const NAME: &'static str = "big-endian unsigned 64-bit integer";

    type Output = u64;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u64::from_be_bytes(bytes))
    }
}
