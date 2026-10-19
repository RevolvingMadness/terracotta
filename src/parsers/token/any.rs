use core::{fmt::Debug, hash::Hash, marker::PhantomData};

use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableDefinition},
    parser::Parser,
    result::ParseResult,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyToken<T> {
    _phantom_data: PhantomData<T>,
}

pub struct ExpectedAnyTokenParseError;

impl<'input, T> ParsableDefinition<'input> for AnyToken<T> {
    type Output = T;

    type Error = ExpectedAnyTokenParseError;

    fn error(&self) -> Self::Error {
        todo!()
    }
}

impl<'input, I: Input + ?Sized + 'input> Parsable<'input, I> for AnyToken<I::Token> {
    fn parse(&self, parser: &mut Parser<I, Self::Error>) -> ParseResult<Self::Output> {
        parser.advance_result()
    }
}
