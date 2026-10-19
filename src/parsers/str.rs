use alloc::{borrow::ToOwned, string::String};

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parser::Parser,
    result::{ParseFailure, ParseResult},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ExpectedStringParseError(String);

impl<'input> ParsableDefinition<'input> for str
where
    Self: 'input,
{
    type Output = &'input Self;

    type Error = ExpectedStringParseError;

    fn error(&self) -> Self::Error {
        ExpectedStringParseError(self.to_owned())
    }
}

impl<'input> Parsable<'input, Self> for str {
    fn parse(&self, parser: &mut Parser<'input, Self, Self::Error>) -> ParseResult<Self::Output> {
        if !parser.slice_starts_with(parser.position(), self) {
            return Err(ParseFailure::Soft);
        }

        let string = parser.advance_len(self.len());

        Ok(string)
    }
}
