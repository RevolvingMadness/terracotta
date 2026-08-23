use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::LittleEndian,
    result::{OptionTExt, ParseResult},
};

pub type LittleEndianF64 = LittleEndian<f64>;

impl Parsable<'_, [u8]> for LittleEndianF64 {
    const NAME: &'static str = "little-endian 64-bit double";

    type Output = f64;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8).into_parse_result_soft()?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_le_bytes(bytes))
    }
}
