use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::LittleEndian,
    result::{OptionTExt, ParseResult},
};

pub type LittleEndianI32 = LittleEndian<i32>;

impl<'input> Parsable<&'input [u8]> for LittleEndianI32 {
    const NAME: &'static str = "little-endian signed 32-bit integer";

    type Output = i32;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4).into_parse_result_soft()?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i32::from_le_bytes(bytes))
    }
}
