use std::num::NonZeroI64;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::{i64::BigEndianI64, non_zero::BigEndianNonZero},
    result::ParseResult,
};

pub type BigEndianNonZeroI64 = BigEndianNonZero<i64>;

impl<'input> Parsable<&'input [u8]> for BigEndianNonZeroI64 {
    const NAME: &'static str = "non-zero big-endian signed 64-bit integer";

    type Output = NonZeroI64;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI64::parse(parser)?;

        let Some(value) = NonZeroI64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero big-endian signed 64-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

impl<'input> Parsable<&'input [u8]> for Option<BigEndianNonZeroI64> {
    const NAME: &'static str = "big-endian signed 64-bit integer";

    type Output = Option<NonZeroI64>;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let value = BigEndianI64::parse(parser)?;

        Ok(NonZeroI64::new(value))
    }
}
