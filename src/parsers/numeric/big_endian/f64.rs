use crate::{
    parsable_traits::Parsable, parser::Parser, parsers::numeric::big_endian::BigEndian,
    result::ParseResult,
};

pub type BigEndianF64 = BigEndian<f64>;

impl Parsable<'_, [u8]> for BigEndianF64 {
    const NAME: &'static str = "big-endian 64-bit double";

    type Output = f64;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_be_bytes(bytes))
    }
}
