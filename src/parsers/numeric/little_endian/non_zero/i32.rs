use std::num::NonZeroI32;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::{i32::LittleEndianI32, non_zero::LittleEndianNonZero},
    result::ParseResult,
};

pub type LittleEndianNonZeroI32 = LittleEndianNonZero<i32>;

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

impl<'input, Context> Parsable<&'input [u8], Context> for Option<LittleEndianNonZeroI32> {
    const NAME: &'static str = "little-endian signed 32-bit integer";

    type Output = Option<NonZeroI32>;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let value = parser.parse::<LittleEndianI32>()?;

        Ok(NonZeroI32::new(value))
    }
}
