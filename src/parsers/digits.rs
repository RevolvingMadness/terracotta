use crate::{parsable_traits::Parsable, parser::Parser, result::regular::ParseResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digits<'input>(pub &'input str);

impl<'input, Context> Parsable<'input, Context> for Digits<'input> {
    const NAME: &'static str = "digits";

    fn parse(parser: &mut Parser<'input, Context>) -> ParseResult<Self> {
        let digits = parser.take_while(|c| c.is_ascii_digit())?;

        ParseResult::Success(Self(digits))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parsers::digits::Digits, result::regular::ParseResult, str::StrExt};

    #[test]
    fn parse() {
        assert_eq!(
            "123".parse_standalone::<Digits>(),
            ParseResult::Success(Digits("123"))
        );

        assert_eq!("abc".parse_standalone::<Digits>(), ParseResult::SoftFailure);
    }
}
