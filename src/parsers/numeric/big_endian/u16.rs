use crate::{
    parsable_traits::Parsable, parser::Parser, parsers::numeric::big_endian::BigEndian,
    result::ParseResult,
};

pub type BigEndianU16 = BigEndian<u16>;

impl Parsable<'_, [u8]> for BigEndianU16 {
    const NAME: &'static str = "big-endian unsigned 16-bit integer";

    type Output = u16;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2)?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u16::from_be_bytes(bytes))
    }
}
