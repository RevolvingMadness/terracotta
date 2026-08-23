use std::num::NonZeroI32;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::{i32::BigEndianI32, non_zero::BigEndianNonZero},
    result::ParseResult,
};

pub type BigEndianNonZeroI32 = BigEndianNonZero<i32>;

impl<'input> Parsable<&'input [u8]> for BigEndianNonZeroI32 {
    const NAME: &'static str = "non-zero big-endian signed 32-bit integer";

    type Output = NonZeroI32;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI32::parse(parser)?;

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

impl<'input> Parsable<&'input [u8]> for Option<BigEndianNonZeroI32> {
    const NAME: &'static str = "big-endian signed 32-bit integer";

    type Output = Option<NonZeroI32>;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let value = BigEndianI32::parse(parser)?;

        Ok(NonZeroI32::new(value))
    }
}
