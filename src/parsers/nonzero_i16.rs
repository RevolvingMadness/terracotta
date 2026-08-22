use std::num::NonZeroI16;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::i16::{BigEndianI16, LittleEndianI16},
    result::ParseResult,
};

pub struct BigEndianNonZeroI16;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianNonZeroI16 {
    const NAME: &'static str = "non-zero big-endian signed 16-bit integer";

    type Output = NonZeroI16;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<BigEndianI16>()?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero big-endian signed 16-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

pub struct LittleEndianNonZeroI16;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianNonZeroI16 {
    const NAME: &'static str = "non-zero little-endian signed 16-bit integer";

    type Output = NonZeroI16;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<LittleEndianI16>()?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero little-endian signed 16-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}
