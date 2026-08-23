use std::num::NonZeroU8;

use crate::{parsable_traits::Parsable, parser::Parser, result::ParseResult};

impl<'input> Parsable<&'input [u8]> for NonZeroU8 {
    const NAME: &'static str = "non-zero unsigned 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = parser.parse::<u8>()?;

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

impl<'input> Parsable<&'input [u8]> for Option<NonZeroU8> {
    const NAME: &'static str = "unsigned 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<&'input [u8]>) -> ParseResult<Self::Output> {
        let value = parser.parse::<u8>()?;

        Ok(NonZeroU8::new(value))
    }
}
