use std::num::NonZeroI8;

use crate::{parsable_traits::Parsable, parser::Parser, result::ParseResult};

impl Parsable<'_, [u8]> for NonZeroI8 {
    const NAME: &'static str = "non-zero signed 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = i8::parse(parser)?;

        let Some(value) = Self::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                "Expected a non-zero signed 8-bit integer".to_owned(),
            );
        };

        Ok(value)
    }
}

impl Parsable<'_, [u8]> for Option<NonZeroI8> {
    const NAME: &'static str = "signed 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let value = i8::parse(parser)?;

        Ok(NonZeroI8::new(value))
    }
}
