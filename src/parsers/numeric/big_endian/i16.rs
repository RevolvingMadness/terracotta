use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::BigEndian,
    result::{OptionTExt, ParseResult},
};

pub type BigEndianI16 = BigEndian<i16>;

impl<'input> Parsable<&'input [u8]> for BigEndianI16 {
    const NAME: &'static str = "big-endian signed 16-bit integer";

    type Output = i16;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2).into_parse_result_soft()?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i16::from_be_bytes(bytes))
    }
}
