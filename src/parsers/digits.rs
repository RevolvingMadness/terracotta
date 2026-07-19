use crate::{parsable_traits::Parsable, parser::Parser, result::ParseResult};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digits;

impl<'input, Context> Parsable<'input, Context> for Digits {
    const NAME: &'static str = "digits";

    type Output = &'input str;

    fn parse(parser: &mut Parser<'input, Context>) -> ParseResult<Self::Output> {
        let digits = parser.take_while(|c| c.is_ascii_digit())?;

        Ok(digits)
    }
}
