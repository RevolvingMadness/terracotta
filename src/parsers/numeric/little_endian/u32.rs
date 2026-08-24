use crate::{
    parsable_traits::Parsable, parser::Parser, parsers::numeric::little_endian::LittleEndian,
    result::ParseResult,
};

pub type LittleEndianU32 = LittleEndian<u32>;

impl Parsable<'_, [u8]> for LittleEndianU32 {
    const NAME: &'static str = "little-endian unsigned 32-bit integer";

    type Output = u32;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u32::from_le_bytes(bytes))
    }
}
