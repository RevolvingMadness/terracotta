use core::{fmt::Debug, hash::Hash, marker::PhantomData};

use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableDefinition},
    parser::Parser,
    result::ParseResult,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyTokenParser<T> {
    _phantom_data: PhantomData<T>,
}

impl<T> AnyTokenParser<T> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _phantom_data: PhantomData,
        }
    }
}

pub struct ExpectedAnyTokenParseError;

impl<T> ParsableDefinition<'_> for AnyTokenParser<T> {
    type Output = T;

    type Error = ExpectedAnyTokenParseError;

    fn error(&self) -> Self::Error {
        todo!()
    }
}

impl<'input, I, E> Parsable<'input, I, E> for AnyTokenParser<I::Token>
where
    I: Input + ?Sized + 'input,
{
    fn parse(&self, parser: &mut Parser<I, E>) -> ParseResult<Self::Output> {
        parser.advance_result()
    }
}
