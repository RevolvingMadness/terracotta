use std::ops::{ControlFlow, FromResidual, Residual, Try};

use crate::{
    parser::Parser,
    result::{
        hard::{HardParseFailure, HardParseResult},
        soft::SoftParseFailure,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseFailure {
    Soft,
    Hard(HardParseFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub enum ParseResult<T> {
    Success(T),
    SoftFailure,
    HardFailure(HardParseFailure),
}

impl<T> Residual<T> for ParseFailure {
    type TryType = ParseResult<T>;
}

impl<T> Try for ParseResult<T> {
    type Output = T;

    type Residual = ParseFailure;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Success(value) => ControlFlow::Continue(value),
            Self::SoftFailure => ControlFlow::Break(ParseFailure::Soft),
            Self::HardFailure(failure) => ControlFlow::Break(ParseFailure::Hard(failure)),
        }
    }
}

impl<T> FromResidual for ParseResult<T> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        match residual {
            ParseFailure::Soft => Self::SoftFailure,
            ParseFailure::Hard(failure) => Self::HardFailure(failure),
        }
    }
}

impl<T> FromResidual<SoftParseFailure> for ParseResult<T> {
    fn from_residual(SoftParseFailure: SoftParseFailure) -> Self {
        Self::SoftFailure
    }
}

impl<T> FromResidual<HardParseFailure> for ParseResult<T> {
    fn from_residual(residual: HardParseFailure) -> Self {
        Self::HardFailure(residual)
    }
}

impl<T> FromResidual<Option<T>> for ParseResult<T> {
    fn from_residual(residual: Option<T>) -> Self {
        match residual {
            Some(value) => Self::Success(value),
            None => Self::SoftFailure,
        }
    }
}

impl<T> FromResidual<Result<T, SoftParseFailure>> for ParseResult<T> {
    fn from_residual(residual: Result<T, SoftParseFailure>) -> Self {
        match residual {
            Ok(value) => Self::Success(value),
            Err(SoftParseFailure) => Self::SoftFailure,
        }
    }
}

impl<T> From<Result<T, HardParseFailure>> for ParseResult<T> {
    fn from(value: Result<T, HardParseFailure>) -> Self {
        match value {
            Ok(value) => Self::Success(value),
            Err(failure) => Self::HardFailure(failure),
        }
    }
}

impl<T> From<Result<Option<T>, HardParseFailure>> for ParseResult<T> {
    fn from(value: Result<Option<T>, HardParseFailure>) -> Self {
        match value {
            Ok(Some(value)) => Self::Success(value),
            Ok(None) => Self::SoftFailure,
            Err(failure) => Self::HardFailure(failure),
        }
    }
}

impl<T> From<ParseResult<T>> for HardParseResult<Option<T>> {
    fn from(value: ParseResult<T>) -> Self {
        match value {
            ParseResult::Success(value) => Self::Success(Some(value)),
            ParseResult::SoftFailure => Self::Success(None),
            ParseResult::HardFailure(failure) => Self::Failure(failure),
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

    #[inline]
    pub fn into_hard_option_parse_result(self) -> HardParseResult<Option<T>> {
        self.into()
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
