use std::num::NonZeroI64;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::{i64::BigEndianI64, non_zero::BigEndianNonZero},
    result::ParseResult,
};

pub type BigEndianNonZeroI64 = BigEndianNonZero<i64>;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianNonZeroI64 {
    const NAME: &'static str = "non-zero big-endian signed 64-bit integer";

    type Output = NonZeroI64;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<BigEndianI64>()?;

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
