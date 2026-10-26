use crate::{
    input::Input,
    parser::Parser,
    results::{ParseResult, option::OptionTExt},
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpectedCharParseError(pub char);

impl ParsableDefinition for char {
    type Output<'input, I: Input + ?Sized + 'input> = Self;

    type Error = ExpectedCharParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedCharParseError(*self)
    }
}

impl<'input, I: Input<Token = Self> + ?Sized + 'input, E> Parsable<'input, I, E> for char {
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output<'input, I>> {
        parser
            .advance_if(|character| character == self)
            .into_parse_result()
    }
}
