use std::ops::{ControlFlow, FromResidual, Residual, Try};

use crate::{
    parser::Parser,
    result::{hard::HardParseResult, regular::ParseResult},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoftParseFailure;

impl<T> Residual<T> for SoftParseFailure {
    type TryType = SoftParseResult<T>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub enum SoftParseResult<T> {
    Success(T),
    Failure,
}

impl<T> Try for SoftParseResult<T> {
    type Output = T;

    type Residual = SoftParseFailure;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Success(value) => ControlFlow::Continue(value),
            Self::Failure => ControlFlow::Break(SoftParseFailure),
        }
    }
}

impl<T> FromResidual for SoftParseResult<T> {
    fn from_residual(SoftParseFailure: <Self as Try>::Residual) -> Self {
        Self::Failure
    }
}

impl<T> From<SoftParseResult<T>> for ParseResult<T> {
    fn from(value: SoftParseResult<T>) -> Self {
        match value {
            SoftParseResult::Success(value) => Self::Success(value),
            SoftParseResult::Failure => Self::SoftFailure,
        }
    }
}

impl<T> From<SoftParseResult<T>> for HardParseResult<Option<T>> {
    fn from(value: SoftParseResult<T>) -> Self {
        match value {
            SoftParseResult::Success(value) => Self::Success(Some(value)),
            SoftParseResult::Failure => Self::Success(None),
        }
    }
}

impl<T> SoftParseResult<T> {
    #[inline]
    pub fn into_parse_result(self) -> ParseResult<T> {
        self.into()
    }

    #[inline]
    pub fn into_hard_option_parse_result(self) -> HardParseResult<Option<T>> {
        self.into()
    }

    pub fn into_hard_parse_result<Context>(
        self,
        parser: &mut Parser<'_, Context>,
        message: impl FnOnce() -> String,
    ) -> HardParseResult<T> {
        match self {
            Self::Success(value) => HardParseResult::Success(value),
            Self::Failure => {
                let message = message();

                let failure = parser.add_error_with_length_one(message);

                HardParseResult::Failure(failure)
            }
        }
    }

    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::Success(value) => value,
            Self::Failure => panic!("called `SoftParseResult::unwrap()` on a `Failure` value"),
        }
    }
}
