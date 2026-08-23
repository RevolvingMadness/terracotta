use std::num::NonZeroU32;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::{non_zero::LittleEndianNonZero, u32::LittleEndianU32},
    result::ParseResult,
};

pub type LittleEndianNonZeroU32 = LittleEndianNonZero<u32>;

impl<'input> Parsable<&'input [u8]> for LittleEndianNonZeroU32 {
    const NAME: &'static str = "non-zero little-endian unsigned 32-bit integer";

    type Output = NonZeroU32;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU32::parse(parser)?;

        let Some(value) = NonZeroU32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero little-endian unsigned 32-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

impl<'input> Parsable<&'input [u8]> for Option<LittleEndianNonZeroU32> {
    const NAME: &'static str = "little-endian unsigned 32-bit integer";

    type Output = Option<NonZeroU32>;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let value = LittleEndianU32::parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}
