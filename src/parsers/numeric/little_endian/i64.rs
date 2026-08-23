use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::LittleEndian,
    result::{OptionTExt, ParseResult},
};

pub type LittleEndianI64 = LittleEndian<i64>;

impl<'input> Parsable<&'input [u8]> for LittleEndianI64 {
    const NAME: &'static str = "little-endian signed 64-bit integer";

    type Output = i64;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i64::from_le_bytes(bytes))
    }
}
