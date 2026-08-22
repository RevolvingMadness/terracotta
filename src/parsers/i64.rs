use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

pub struct BigEndianI64;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianI64 {
    const NAME: &'static str = "big-endian signed 64-bit integer";

    type Output = i64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i64::from_be_bytes(bytes))
    }
}

pub struct LittleEndianI64;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianI64 {
    const NAME: &'static str = "little-endian signed 64-bit integer";

    type Output = i64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i64::from_le_bytes(bytes))
    }
}
