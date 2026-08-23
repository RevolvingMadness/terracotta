use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::{OptionTExt, ParseResult},
};

pub type BigEndianI32 = BigEndian<i32>;

impl<'input> Parsable<&'input [u8]> for BigEndianI32 {
    const NAME: &'static str = "big-endian signed 32-bit integer";

    type Output = i32;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4).into_parse_result_soft()?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i32::from_be_bytes(bytes))
    }
}
