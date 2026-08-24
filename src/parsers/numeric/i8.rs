use crate::{parsable_traits::Parsable, parser::Parser, result::ParseResult};

impl Parsable<'_, [u8]> for i8 {
    const NAME: &'static str = "signed 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<[u8]>) -> ParseResult<Self::Output> {
        let value = parser.advance()?;

        Ok(value as Self)
    }
}
