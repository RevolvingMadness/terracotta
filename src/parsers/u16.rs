use crate::{
    parsable_traits::Parsable,
    parser::{CalledFromParser, Parser},
    result::{OptionTExt, ParseResult},
};

pub struct BigEndianU16;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianU16 {
    const NAME: &'static str = "big-endian unsigned 16-bit integer";

    type Output = u16;

    fn parse(
        parser: &mut Parser<&'input [u8], Context>,
        _: CalledFromParser,
    ) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact::<2>().into_parse_result_soft()?;

        Ok(u16::from_be_bytes(bytes))
    }
}
