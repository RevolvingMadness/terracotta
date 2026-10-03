use core::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{input::Input, parsable_traits::Parsable, parser::Parser, result::ParseResult};

pub struct AnyToken<T>(PhantomData<T>);

impl<T> Debug for AnyToken<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyToken").finish()
    }
}

impl<T> Clone for AnyToken<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AnyToken<T> {}

impl<T> Default for AnyToken<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> PartialEq for AnyToken<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for AnyToken<T> {}

impl<T> PartialOrd for AnyToken<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for AnyToken<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for AnyToken<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<I: Input + ?Sized, E> Parsable<'_, I, E> for AnyToken<I::Token> {
    type Output = I::Token;

    fn parse(parser: &mut Parser<I, E>) -> ParseResult<Self::Output> {
        parser.advance_result()
    }
}
