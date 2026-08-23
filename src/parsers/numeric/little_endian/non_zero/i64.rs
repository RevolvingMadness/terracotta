use std::num::NonZeroI64;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::{i64::LittleEndianI64, non_zero::LittleEndianNonZero},
    result::ParseResult,
};

pub type LittleEndianNonZeroI64 = LittleEndianNonZero<i64>;

impl<'input, Context> Parsable<&'input [u8], Context> for LittleEndianNonZeroI64 {
    const NAME: &'static str = "non-zero little-endian signed 64-bit integer";

    type Output = NonZeroI64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<LittleEndianI64>()?;

        let Some(value) = NonZeroI64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero little-endian signed 64-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}
