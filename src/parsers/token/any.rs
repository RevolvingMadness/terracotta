use crate::{
    input::Input,
    parser::Parser,
    results::{ParseResult, option::OptionTExt},
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct AnyTokenParser;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExpectedAnyTokenParseError;

impl ParsableDefinition for AnyTokenParser {
    type Output<'input, I: Input + ?Sized + 'input> = I::Token;

    type Error = ExpectedAnyTokenParseError;

    fn error(&self) -> Self::Error {
        ExpectedAnyTokenParseError
    }
}

impl<'input, I, E> Parsable<'input, I, E> for AnyTokenParser
where
    I: Input + ?Sized + 'input,
{
    fn parse(&self, parser: &mut Parser<I, E>) -> ParseResult<Self::Output<'input, I>> {
        parser.advance().into_parse_result()
    }
}
