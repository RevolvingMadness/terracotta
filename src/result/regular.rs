use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Residual, Try},
};

use crate::{
    parser::{ParseFailure, Parser},
    result::{
        hard::{HardParseResult, HardParseResultResidual},
        soft::{SoftParseResult, SoftParseResultResidual},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub enum ParseResult<T> {
    Success(T),
    SoftFailure,
    HardFailure(ParseFailure),
}

pub(crate) type ParseResultResidual = ParseResult<Infallible>;

impl<T> Residual<T> for ParseResultResidual {
    type TryType = ParseResult<T>;
}

impl<T> Try for ParseResult<T> {
    type Output = T;

    type Residual = ParseResultResidual;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Success(value) => ControlFlow::Continue(value),
            Self::SoftFailure => ControlFlow::Break(ParseResult::SoftFailure),
            Self::HardFailure(failure) => ControlFlow::Break(ParseResult::HardFailure(failure)),
        }
    }
}

impl<T> FromResidual<ParseResultResidual> for ParseResult<T> {
    fn from_residual(residual: ParseResultResidual) -> Self {
        match residual {
            ParseResult::Success(..) => unreachable!(),
            ParseResult::SoftFailure => Self::SoftFailure,
            ParseResult::HardFailure(failure) => Self::HardFailure(failure),
        }
    }
}

impl<T> FromResidual<SoftParseResultResidual> for ParseResult<T> {
    fn from_residual(residual: SoftParseResultResidual) -> Self {
        match residual {
            SoftParseResult::Success(..) => unreachable!(),
            SoftParseResult::Failure => Self::SoftFailure,
        }
    }
}

impl<T> FromResidual<HardParseResultResidual> for ParseResult<T> {
    fn from_residual(residual: HardParseResultResidual) -> Self {
        match residual {
            HardParseResult::Success(..) => unreachable!(),
            HardParseResult::Failure(failure) => Self::HardFailure(failure),
        }
    }
}

impl<T> ParseResult<T> {
    pub fn into_hard_parse_result<Context>(
        self,
        parser: &mut Parser<'_, Context>,
        message: impl FnOnce() -> String,
    ) -> HardParseResult<T> {
        match self {
            Self::Success(value) => HardParseResult::Success(value),
            Self::SoftFailure => {
                let message = message();

                let failure = parser.add_error_with_length_one(message);

                HardParseResult::Failure(failure)
            }
            Self::HardFailure(failure) => HardParseResult::Failure(failure),
        }
    }

    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::Success(value) => value,
            Self::SoftFailure => panic!("called `ParseResult::unwrap()` on a `SoftFailure` value"),
            Self::HardFailure(..) => {
                panic!("called `ParseResult::unwrap()` on a `HardFailure` value")
            }
        }
    }
}
