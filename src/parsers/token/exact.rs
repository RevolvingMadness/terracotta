use core::{fmt::Debug, hash::Hash};

use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableDefinition},
    parser::Parser,
    result::{ParseFailure, ParseResult},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExactTokenParser<T: PartialEq> {
    token: T,
}

impl<T: PartialEq> ExactTokenParser<T> {
    #[inline]
    #[must_use]
    pub const fn new(token: T) -> Self {
        Self { token }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpectedExactTokenParseError<T: PartialEq> {
    token: T,
}

impl<T: PartialEq> ExpectedExactTokenParseError<T> {
    #[inline]
    #[must_use]
    pub const fn new(token: T) -> Self {
        Self { token }
    }
}

impl<T: PartialEq + Clone> ParsableDefinition<'_> for ExactTokenParser<T> {
    type Output = T;

    type Error = ExpectedExactTokenParseError<T>;

    fn error(&self) -> Self::Error {
        ExpectedExactTokenParseError::new(self.token.clone())
    }
}

impl<'input, I: Input + ?Sized + 'input> Parsable<'input, I> for ExactTokenParser<I::Token>
where
    I::Token: PartialEq,
{
    fn parse(&self, parser: &mut Parser<'input, I, Self::Error>) -> ParseResult<Self::Output> {
        parser
            .advance_if_result(|token| *token == self.token)
            .and_then(|token| token.ok_or(ParseFailure::Soft))
    }
}
