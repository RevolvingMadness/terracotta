use std::num::NonZeroU32;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::{non_zero::BigEndianNonZero, u32::BigEndianU32},
    result::ParseResult,
};

pub type BigEndianNonZeroU32 = BigEndianNonZero<u32>;

impl<'input> Parsable<&'input [u8]> for BigEndianNonZeroU32 {
    const NAME: &'static str = "non-zero big-endian unsigned 32-bit integer";

    type Output = NonZeroU32;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU32::parse(parser)?;

        let Some(value) = NonZeroU32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero big-endian unsigned 32-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

impl<'input> Parsable<&'input [u8]> for Option<BigEndianNonZeroU32> {
    const NAME: &'static str = "big-endian unsigned 32-bit integer";

    type Output = Option<NonZeroU32>;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let value = BigEndianU32::parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}
