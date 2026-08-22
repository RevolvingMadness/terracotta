use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

pub struct BigEndianF32;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianF32 {
    const NAME: &'static str = "big-endian 32-bit float";

    type Output = f32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4).into_parse_result_soft()?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f32::from_be_bytes(bytes))
    }
}

pub struct LittleEndianF32;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianF32 {
    const NAME: &'static str = "little-endian 32-bit float";

    type Output = f32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4).into_parse_result_soft()?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f32::from_le_bytes(bytes))
    }
}
