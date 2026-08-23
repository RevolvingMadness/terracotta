use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::{OptionTExt, ParseResult},
};

pub type BigEndianU32 = BigEndian<u32>;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianU32 {
    const NAME: &'static str = "big-endian unsigned 32-bit integer";

    type Output = u32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4).into_parse_result_soft()?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u32::from_be_bytes(bytes))
    }
}
