use std::num::NonZeroU8;

use crate::{parsable_traits::Parsable, parser::Parser, result::ParseResult};

impl Parsable<'_, [u8]> for NonZeroU8 {
    const NAME: &'static str = "non-zero unsigned 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = u8::parse(parser)?;

        let Some(value) = Self::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero unsigned 8-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

impl Parsable<'_, [u8]> for Option<NonZeroU8> {
    const NAME: &'static str = "unsigned 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let value = u8::parse(parser)?;

        Ok(NonZeroU8::new(value))
    }
}
