use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

pub struct BigEndianU64;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianU64 {
    const NAME: &'static str = "big-endian unsigned 64-bit integer";

    type Output = u64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u64::from_be_bytes(bytes))
    }
}

pub struct LittleEndianU64;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianU64 {
    const NAME: &'static str = "little-endian unsigned 64-bit integer";

    type Output = u64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u64::from_le_bytes(bytes))
    }
}
