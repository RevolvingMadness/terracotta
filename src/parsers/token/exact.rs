use core::{fmt::Debug, hash::Hash};

use crate::{
    failures::ParseFailure,
    input::Input,
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExactTokenParser<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpectedExactTokenParseError<T>(pub T);

impl<T: Clone> ParsableDefinition<'_> for ExactTokenParser<T> {
    type Output = T;

    type Error = ExpectedExactTokenParseError<T>;

    fn error(&self) -> Self::Error {
        ExpectedExactTokenParseError(self.0.clone())
    }
}

impl<'input, I, E> Parsable<'input, I, E> for ExactTokenParser<I::Token>
where
    I: Input + ?Sized + 'input,
{
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output> {
        parser
            .advance_if_result(|token| *token == self.0)
            .and_then(|token| token.ok_or(ParseFailure::Soft))
    }
}
