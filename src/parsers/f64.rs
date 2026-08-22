use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

pub struct BigEndianF64;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianF64 {
    const NAME: &'static str = "big-endian 64-bit double";

    type Output = f64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_be_bytes(bytes))
    }
}

pub struct LittleEndianF64;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianF64 {
    const NAME: &'static str = "little-endian 64-bit double";

    type Output = f64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_le_bytes(bytes))
    }
}
