use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

pub struct BigEndianI16;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianI16 {
    const NAME: &'static str = "big-endian signed 16-bit integer";

    type Output = i16;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2).into_parse_result_soft()?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i16::from_be_bytes(bytes))
    }
}

pub struct LittleEndianI16;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianI16 {
    const NAME: &'static str = "little-endian signed 16-bit integer";

    type Output = i16;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2).into_parse_result_soft()?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i16::from_le_bytes(bytes))
    }
}
