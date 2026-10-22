use core::fmt::{self, Display, Formatter};

use alloc::string::String;

use crate::{
    errors::{end_of_input::ExpectedEndOfInputParseError, number::ExpectedNumberParseError},
    numeric_expectation::NumericExpectation,
    parsers::{
        str::ExpectedStringParseError,
        token::{any::ExpectedAnyTokenParseError, exact::ExpectedExactTokenParseError},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefaultParseError<T> {
    ExpectedEndOfInput,
    ExpectedString(String),
    ExpectedAnyToken,
    ExpectedExactToken(T),
    Expected(String),
    ExpectedNumber(NumericExpectation),
}

impl<T: Display> Display for DefaultParseError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedEndOfInput => write!(f, "expected end of input"),
            Self::ExpectedString(string) => write!(f, r#"expected "{}""#, string),
            Self::ExpectedAnyToken => write!(f, "expected any character"),
            Self::ExpectedExactToken(character) => write!(f, "expected '{}'", character),
            Self::Expected(expectation) => write!(f, "expected {}", expectation),
            Self::ExpectedNumber(expectation) => write!(f, "expected a {}", expectation),
        }
    }
}

impl<T> From<ExpectedEndOfInputParseError> for DefaultParseError<T> {
    fn from(_: ExpectedEndOfInputParseError) -> Self {
        Self::ExpectedEndOfInput
    }
}

impl<T> From<ExpectedStringParseError> for DefaultParseError<T> {
    fn from(ExpectedStringParseError(value): ExpectedStringParseError) -> Self {
        Self::ExpectedString(value)
    }
}

impl<T> From<ExpectedAnyTokenParseError> for DefaultParseError<T> {
    fn from(_: ExpectedAnyTokenParseError) -> Self {
        Self::ExpectedAnyToken
    }
}

impl<T> From<ExpectedExactTokenParseError<T>> for DefaultParseError<T> {
    fn from(ExpectedExactTokenParseError(token): ExpectedExactTokenParseError<T>) -> Self {
        Self::ExpectedExactToken(token)
    }
}

impl<T> From<ExpectedNumberParseError> for DefaultParseError<T> {
    fn from(ExpectedNumberParseError(expectation): ExpectedNumberParseError) -> Self {
        Self::ExpectedNumber(expectation)
    }
}
