use std::num::NonZeroU32;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::u32::{BigEndianU32, LittleEndianU32},
    result::ParseResult,
};

pub struct BigEndianNonZeroU32;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianNonZeroU32 {
    const NAME: &'static str = "non-zero big-endian unsigned 32-bit integer";

    type Output = NonZeroU32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<BigEndianU32>()?;

        let Some(value) = NonZeroU32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero big-endian unsigned 32-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

pub struct LittleEndianNonZeroU32;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianNonZeroU32 {
    const NAME: &'static str = "non-zero little-endian unsigned 32-bit integer";

    type Output = NonZeroU32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<LittleEndianU32>()?;

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
