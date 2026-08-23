use std::num::NonZeroI16;

use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    parsers::numeric::little_endian::{i16::LittleEndianI16, non_zero::LittleEndianNonZero},
    result::ParseResult,
};

pub type LittleEndianNonZeroI16 = LittleEndianNonZero<i16>;

impl Parsable<'_, [u8]> for LittleEndianNonZeroI16 {
    const NAME: &'static str = "non-zero little-endian signed 16-bit integer";

    type Output = NonZeroI16;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI16::parse(parser)?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero little-endian signed 16-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

impl Parsable<'_, [u8]> for Option<LittleEndianNonZeroI16> {
    const NAME: &'static str = "little-endian signed 16-bit integer";

    type Output = Option<NonZeroI16>;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let value = LittleEndianI16::parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}
