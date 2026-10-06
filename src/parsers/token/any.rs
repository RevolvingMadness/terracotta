use core::{fmt::Debug, hash::Hash};

use crate::{input::Input, parsable_traits::Parsable, parser::Parser, result::ParseResult};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyToken;

impl<I: Input + ?Sized, E> Parsable<'_, I, E> for AnyToken {
    type Output = I::Token;

    fn parse(parser: &mut Parser<I, E>) -> ParseResult<Self::Output> {
        parser.advance_result()
    }
}
