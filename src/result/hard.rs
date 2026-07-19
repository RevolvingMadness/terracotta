use std::ops::{ControlFlow, FromResidual, Residual, Try};

use crate::result::regular::ParseResult;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HardParseFailure(pub(crate) ());

impl<T> Residual<T> for HardParseFailure {
    type TryType = HardParseResult<T>;
}

impl HardParseFailure {
    #[inline]
    #[must_use]
    pub const fn new_unchecked() -> Self {
        Self(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub enum HardParseResult<T> {
    Success(T),
    Failure(HardParseFailure),
}

impl<T> Try for HardParseResult<T> {
    type Output = T;

    type Residual = HardParseFailure;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Success(value) => ControlFlow::Continue(value),
            Self::Failure(failure) => ControlFlow::Break(failure),
        }
    }
}

impl<T> FromResidual for HardParseResult<T> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        Self::Failure(residual)
    }
}

impl<T> From<HardParseResult<T>> for ParseResult<T> {
    fn from(value: HardParseResult<T>) -> Self {
        match value {
            HardParseResult::Success(value) => Self::Success(value),
            HardParseResult::Failure(failure) => Self::HardFailure(failure),
        }
    }
}

impl<T> From<HardParseResult<Option<T>>> for ParseResult<T> {
    fn from(value: HardParseResult<Option<T>>) -> Self {
        match value {
            HardParseResult::Success(Some(value)) => Self::Success(value),
            HardParseResult::Success(None) => Self::SoftFailure,
            HardParseResult::Failure(failure) => Self::HardFailure(failure),
        }
    }
}

impl<T> HardParseResult<T> {
    #[inline]
    pub fn into_parse_result(self) -> ParseResult<T> {
        self.into()
    }

    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::Success(value) => value,
            Self::Failure(..) => panic!("called `HardParseResult::unwrap()` on a `Failure` value"),
        }
    }
}
