use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

pub struct BigEndianI32;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianI32 {
    const NAME: &'static str = "big-endian signed 32-bit integer";

    type Output = i32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4).into_parse_result_soft()?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i32::from_be_bytes(bytes))
    }
}

pub struct LittleEndianI32;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianI32 {
    const NAME: &'static str = "little-endian signed 32-bit integer";

    type Output = i32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4).into_parse_result_soft()?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i32::from_le_bytes(bytes))
    }
}
