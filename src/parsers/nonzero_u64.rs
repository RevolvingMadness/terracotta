use std::num::NonZeroU64;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::u64::{BigEndianU64, LittleEndianU64},
    result::ParseResult,
};

pub struct BigEndianNonZeroU64;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianNonZeroU64 {
    const NAME: &'static str = "non-zero big-endian unsigned 64-bit integer";

    type Output = NonZeroU64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<BigEndianU64>()?;

        let Some(value) = NonZeroU64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero big-endian unsigned 64-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

pub struct LittleEndianNonZeroU64;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianNonZeroU64 {
    const NAME: &'static str = "non-zero little-endian unsigned 64-bit integer";

    type Output = NonZeroU64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<LittleEndianU64>()?;

        let Some(value) = NonZeroU64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero little-endian unsigned 64-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}
