use std::num::NonZeroU16;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::{non_zero::LittleEndianNonZero, u16::LittleEndianU16},
    result::ParseResult,
};

pub type LittleEndianNonZeroU16 = LittleEndianNonZero<u16>;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianNonZeroU16 {
    const NAME: &'static str = "non-zero little-endian unsigned 16-bit integer";

    type Output = NonZeroU16;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<LittleEndianU16>()?;

        let Some(value) = NonZeroU16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero little-endian unsigned 16-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

impl<'input, Context> Parsable<&'input [u8], Context> for Option<LittleEndianNonZeroU16> {
    const NAME: &'static str = "little-endian unsigned 16-bit integer";

    type Output = Option<NonZeroU16>;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let value = parser.parse::<LittleEndianU16>()?;

        Ok(NonZeroU16::new(value))
    }
}
