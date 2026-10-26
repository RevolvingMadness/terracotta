use crate::{
    input::Input,
    parser::Parser,
    results::{ParseResult, option::OptionTExt},
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExpectedAnyCharParseError;

pub struct CharParser;

impl ParsableDefinition for CharParser {
    type Output<'input, I: Input + ?Sized + 'input> = char;

    type Error = ExpectedAnyCharParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedAnyCharParseError
    }
}

impl<'input, I: Input<Token = char> + ?Sized + 'input, E> Parsable<'input, I, E> for CharParser {
    #[inline]
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output<'input, I>> {
        parser.advance().into_parse_result()
    }
}
