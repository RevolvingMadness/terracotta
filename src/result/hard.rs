use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Residual, Try},
};

use crate::{parser::ParseFailure, result::regular::ParseResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub enum HardParseResult<T> {
    Success(T),
    Failure(ParseFailure),
}

pub(crate) type HardParseResultResidual = HardParseResult<Infallible>;

impl<T> Residual<T> for HardParseResultResidual {
    type TryType = HardParseResult<T>;
}

impl<T> Try for HardParseResult<T> {
    type Output = T;

    type Residual = HardParseResultResidual;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Success(value) => ControlFlow::Continue(value),
            Self::Failure(failure) => ControlFlow::Break(HardParseResult::Failure(failure)),
        }
    }
}

impl<T> FromResidual<HardParseResultResidual> for HardParseResult<T> {
    fn from_residual(residual: HardParseResultResidual) -> Self {
        match residual {
            HardParseResult::Success(..) => unreachable!(),
            HardParseResult::Failure(failure) => Self::Failure(failure),
        }
    }
}

impl<T> HardParseResult<T> {
    pub fn into_parse_result(self) -> ParseResult<T> {
        match self {
            Self::Success(value) => ParseResult::Success(value),
            Self::Failure(failure) => ParseResult::HardFailure(failure),
        }
    }

    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::Success(value) => value,
            Self::Failure(..) => panic!("called `HardParseResult::unwrap()` on a `Failure` value"),
        }
    }
}
