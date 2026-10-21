use core::fmt::{self, Display, Formatter};

use alloc::string::String;

use crate::parsers::{
    str::ExpectedStringParseError,
    token::{any::ExpectedAnyTokenParseError, exact::ExpectedExactTokenParseError},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endianness {
    Little,
    Big,
}

pub struct ExpectedEndOfInputParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericExpectation {
    Byte {
        signed: bool,
        non_zero: bool,
    },
    Short {
        signed: bool,
        non_zero: bool,
        endianness: Endianness,
    },
    Integer {
        signed: bool,
        non_zero: bool,
        endianness: Endianness,
    },
    Long {
        signed: bool,
        non_zero: bool,
        endianness: Endianness,
    },
    Float {
        endianness: Endianness,
    },
    Double {
        endianness: Endianness,
    },
}

impl Display for NumericExpectation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Byte { signed, non_zero } => {
                if non_zero {
                    write!(f, "non-zero ")?;
                }

                if signed {
                    write!(f, "signed ")?;
                } else {
                    write!(f, "unsigned ")?;
                }

                write!(f, "8-bit integer")
            }
            Self::Short {
                signed,
                non_zero,
                endianness,
            } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian ")?;

                if non_zero {
                    write!(f, "non-zero ")?;
                }

                if signed {
                    write!(f, "signed ")?;
                } else {
                    write!(f, "unsigned ")?;
                }

                write!(f, "8-bit integer")
            }
            Self::Integer {
                signed,
                non_zero,
                endianness,
            } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian ")?;

                if non_zero {
                    write!(f, "non-zero ")?;
                }

                if signed {
                    write!(f, "signed ")?;
                } else {
                    write!(f, "unsigned ")?;
                }

                write!(f, "8-bit integer")
            }
            Self::Long {
                signed,
                non_zero,
                endianness,
            } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian ")?;

                if non_zero {
                    write!(f, "non-zero ")?;
                }

                if signed {
                    write!(f, "signed ")?;
                } else {
                    write!(f, "unsigned ")?;
                }

                write!(f, "8-bit integer")
            }
            Self::Float { endianness } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian float")
            }
            Self::Double { endianness } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian float")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpectedNumberParseError(pub NumericExpectation);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefaultParseError<T> {
    ExpectedEndOfInput,
    ExpectedString(String),
    ExpectedAnyToken,
    ExpectedExactToken(T),
    Expected(String),
    Number(NumericExpectation),
}

impl<T: Display> Display for DefaultParseError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedEndOfInput => write!(f, "expected end of input"),
            Self::ExpectedString(string) => write!(f, r#"expected "{}""#, string),
            Self::ExpectedAnyToken => write!(f, "expected any character"),
            Self::ExpectedExactToken(character) => write!(f, "expected '{}'", character),
            Self::Expected(expectation) => write!(f, "expected {}", expectation),
            Self::Number(expectation) => write!(f, "expected a {}", expectation),
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
        Self::Number(expectation)
    }
}
