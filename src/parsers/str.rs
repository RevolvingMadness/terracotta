use alloc::{borrow::ToOwned, string::String};

use crate::{
    failures::ParseFailure,
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ExpectedStringParseError(pub String);

impl<'input> ParsableDefinition<'input> for str {
    type Output = &'input Self;

    type Error = ExpectedStringParseError;

    fn error(&self) -> Self::Error {
        ExpectedStringParseError(self.to_owned())
    }
}

impl<'input, E: From<Self::Error>> Parsable<'input, Self, E> for str {
    fn parse(&self, parser: &mut Parser<'input, Self, E>) -> ParseResult<Self::Output> {
        if !parser.slice_starts_with(parser.position(), self) {
            return Err(ParseFailure::Soft);
        }

        let string = parser.advance_len(self.len());

        Ok(string)
    }
}
