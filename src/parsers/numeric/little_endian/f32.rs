use crate::{
    parsable_traits::Parsable, parser::Parser, parsers::numeric::little_endian::LittleEndian,
    result::ParseResult,
};

pub type LittleEndianF32 = LittleEndian<f32>;

impl Parsable<'_, [u8]> for LittleEndianF32 {
    const NAME: &'static str = "little-endian 32-bit float";

    type Output = f32;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f32::from_le_bytes(bytes))
    }
}
