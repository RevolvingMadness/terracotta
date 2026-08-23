use std::num::NonZeroU16;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::{non_zero::BigEndianNonZero, u16::BigEndianU16},
    result::ParseResult,
};

pub type BigEndianNonZeroU16 = BigEndianNonZero<u16>;

impl<'input, Context> Parsable<&'input [u8], Context> for BigEndianNonZeroU16 {
    const NAME: &'static str = "non-zero big-endian unsigned 16-bit integer";

    type Output = NonZeroU16;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<BigEndianU16>()?;

        let Some(value) = NonZeroU16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero big-endian unsigned 16-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

impl<'input, Context> Parsable<&'input [u8], Context> for Option<BigEndianNonZeroU16> {
    const NAME: &'static str = "big-endian unsigned 16-bit integer";

    type Output = Option<NonZeroU16>;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let value = parser.parse::<BigEndianU16>()?;

        Ok(NonZeroU16::new(value))
    }
}
