use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::{OptionTExt, ParseResult},
};

pub type BigEndianI64 = BigEndian<i64>;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianI64 {
    const NAME: &'static str = "big-endian signed 64-bit integer";

    type Output = i64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i64::from_be_bytes(bytes))
    }
}
