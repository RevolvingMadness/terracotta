use crate::{parsable_traits::Parsable, parser::Parser, result::regular::ParseResult};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digits;

impl<'input, Context> Parsable<'input, Context> for Digits {
    const NAME: &'static str = "digits";

    type Output = &'input str;

    fn parse(parser: &mut Parser<'input, Context>) -> ParseResult<Self::Output> {
        let digits = parser.take_while(|c| c.is_ascii_digit())?;

        ParseResult::Success(digits)
    }
}

#[cfg(test)]
mod tests {
    use crate::{parsers::digits::Digits, result::hard::HardParseResult, str::StrExt};

    #[test]
    fn parse() {
        assert_eq!(
            "123".parse_standalone::<Digits>(false).1,
            HardParseResult::Success(Some("123"))
        );

        assert_eq!(
            "abc".parse_standalone::<Digits>(false).1,
            HardParseResult::Success(None)
        );
    }
}
