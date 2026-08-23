use std::num::NonZeroU64;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::{non_zero::LittleEndianNonZero, u64::LittleEndianU64},
    result::ParseResult,
};

pub type LittleEndianNonZeroU64 = LittleEndianNonZero<u64>;

impl<'input> Parsable<&'input [u8]> for LittleEndianNonZeroU64 {
    const NAME: &'static str = "non-zero little-endian unsigned 64-bit integer";

    type Output = NonZeroU64;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU64::parse(parser)?;

        let Some(value) = NonZeroU64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero little-endian unsigned 64-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

impl<'input> Parsable<&'input [u8]> for Option<LittleEndianNonZeroU64> {
    const NAME: &'static str = "little-endian unsigned 64-bit integer";

    type Output = Option<NonZeroU64>;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let value = LittleEndianU64::parse(parser)?;

        Ok(NonZeroU64::new(value))
    }
}
