use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::{OptionTExt, ParseResult},
};

pub type BigEndianU64 = BigEndian<u64>;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianU64 {
    const NAME: &'static str = "big-endian unsigned 64-bit integer";

    type Output = u64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u64::from_be_bytes(bytes))
    }
}
