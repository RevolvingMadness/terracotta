use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::{OptionTExt, ParseResult},
};

pub type BigEndianF32 = BigEndian<f32>;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianF32 {
    const NAME: &'static str = "big-endian 32-bit float";

    type Output = f32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4).into_parse_result_soft()?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f32::from_be_bytes(bytes))
    }
}
