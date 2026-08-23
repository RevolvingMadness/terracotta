use std::num::NonZeroI64;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::{i64::LittleEndianI64, non_zero::LittleEndianNonZero},
    result::ParseResult,
};

pub type LittleEndianNonZeroI64 = LittleEndianNonZero<i64>;

impl<'input> Parsable<&'input [u8]> for LittleEndianNonZeroI64 {
    const NAME: &'static str = "non-zero little-endian signed 64-bit integer";

    type Output = NonZeroI64;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI64::parse(parser)?;

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

impl<'input> Parsable<&'input [u8]> for Option<LittleEndianNonZeroI64> {
    const NAME: &'static str = "little-endian signed 64-bit integer";

    type Output = Option<NonZeroI64>;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let value = LittleEndianI64::parse(parser)?;

        Ok(NonZeroI64::new(value))
    }
}
