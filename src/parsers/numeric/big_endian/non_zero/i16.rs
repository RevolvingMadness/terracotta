use std::num::NonZeroI16;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::big_endian::{i16::BigEndianI16, non_zero::BigEndianNonZero},
    result::ParseResult,
};

pub type BigEndianNonZeroI16 = BigEndianNonZero<i16>;

impl Parsable<'_, [u8]> for BigEndianNonZeroI16 {
    const NAME: &'static str = "non-zero big-endian signed 16-bit integer";

    type Output = NonZeroI16;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI16::parse(parser)?;

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

impl Parsable<'_, [u8]> for Option<BigEndianNonZeroI16> {
    const NAME: &'static str = "big-endian signed 16-bit integer";

    type Output = Option<NonZeroI16>;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let value = BigEndianI16::parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}
