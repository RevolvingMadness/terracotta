use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::LittleEndian,
    result::{OptionTExt, ParseResult},
};

pub type LittleEndianU16 = LittleEndian<u16>;

impl<'input> Parsable<&'input [u8]> for LittleEndianU16 {
    const NAME: &'static str = "little-endian unsigned 16-bit integer";

    type Output = u16;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2).into_parse_result_soft()?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u16::from_le_bytes(bytes))
    }
}
