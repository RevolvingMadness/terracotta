use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

pub struct BigEndianU32;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianU32 {
    const NAME: &'static str = "big-endian unsigned 32-bit integer";

    type Output = u32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact::<4>().into_parse_result_soft()?;

        Ok(u32::from_be_bytes(bytes))
    }
}

pub struct LittleEndianU32;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianU32 {
    const NAME: &'static str = "little-endian unsigned 32-bit integer";

    type Output = u32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact::<4>().into_parse_result_soft()?;

        Ok(u32::from_le_bytes(bytes))
    }
}
