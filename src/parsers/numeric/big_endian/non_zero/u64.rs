use std::num::NonZeroU64;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::{non_zero::BigEndianNonZero, u64::BigEndianU64},
    result::ParseResult,
};

pub type BigEndianNonZeroU64 = BigEndianNonZero<u64>;

impl<'input> Parsable<&'input [u8]> for BigEndianNonZeroU64 {
    const NAME: &'static str = "non-zero big-endian unsigned 64-bit integer";

    type Output = NonZeroU64;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
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

impl<'input> Parsable<&'input [u8]> for Option<BigEndianNonZeroU64> {
    const NAME: &'static str = "big-endian unsigned 64-bit integer";

    type Output = Option<NonZeroU64>;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let value = parser.parse::<BigEndianU64>()?;

        Ok(NonZeroU64::new(value))
    }
}
