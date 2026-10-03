use core::{fmt::Debug, hash::Hash};

use crate::{
    input::Input,
    parsable_traits::ParsableInstance,
    parser::Parser,
    result::{ParseFailure, ParseResult},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExactToken<T>(pub T);

impl<I: Input + ?Sized, E> ParsableInstance<'_, I, E> for ExactToken<I::Token>
where
    I::Token: PartialEq,
{
    type Output = I::Token;

    fn instance_parse(&self, parser: &mut Parser<I, E>) -> ParseResult<Self::Output> {
        parser
            .advance_if_result(|token| *token == self.0)
            .and_then(|token| token.ok_or(ParseFailure::Soft))
    }
}
