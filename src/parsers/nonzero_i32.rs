use std::num::NonZeroI32;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::i32::{BigEndianI32, LittleEndianI32},
    result::ParseResult,
};

pub struct BigEndianNonZeroI32;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianNonZeroI32 {
    const NAME: &'static str = "non-zero big-endian signed 32-bit integer";

    type Output = NonZeroI32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<BigEndianI32>()?;

        let Some(value) = NonZeroI32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero big-endian signed 32-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

pub struct LittleEndianNonZeroI32;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianNonZeroI32 {
    const NAME: &'static str = "non-zero little-endian signed 32-bit integer";

    type Output = NonZeroI32;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<LittleEndianI32>()?;

        let Some(value) = NonZeroI32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero little-endian signed 32-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}
