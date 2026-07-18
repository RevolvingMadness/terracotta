use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Residual, Try},
};

use crate::{
    parser::Parser,
    result::{hard::HardParseResult, regular::ParseResult},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub enum SoftParseResult<T> {
    Success(T),
    Failure,
}

pub(crate) type SoftParseResultResidual = SoftParseResult<Infallible>;

impl<T> Residual<T> for SoftParseResultResidual {
    type TryType = SoftParseResult<T>;
}

impl<T> Try for SoftParseResult<T> {
    type Output = T;

    type Residual = SoftParseResultResidual;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Success(value) => ControlFlow::Continue(value),
            Self::Failure => ControlFlow::Break(SoftParseResult::Failure),
        }
    }
}

impl<T> FromResidual<SoftParseResultResidual> for SoftParseResult<T> {
    fn from_residual(residual: SoftParseResultResidual) -> Self {
        match residual {
            SoftParseResult::Success(..) => unreachable!(),
            SoftParseResult::Failure => Self::Failure,
        }
    }
}

impl<T> SoftParseResult<T> {
    pub fn into_parse_result(self) -> ParseResult<T> {
        match self {
            Self::Success(value) => ParseResult::Success(value),
            Self::Failure => ParseResult::SoftFailure,
        }
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
