use core::fmt::{self, Display, Formatter};

use alloc::{
    borrow::{Cow, ToOwned},
    format,
    string::String,
};

pub trait ExpectedEndOfInputParseError {
    #[must_use]
    fn expected_end_of_input() -> Self;
}

impl ExpectedEndOfInputParseError for String {
    #[inline]
    fn expected_end_of_input() -> Self {
        "expected end of input".to_owned()
    }
}

impl ExpectedEndOfInputParseError for () {
    #[inline]
    fn expected_end_of_input() -> Self {}
}

impl<T> ExpectedEndOfInputParseError for DefaultParseError<T> {
    #[inline]
    fn expected_end_of_input() -> Self {
        Self::ExpectedEndOfInput
    }
}

pub trait ExpectedParseError {
    #[must_use]
    fn expected(expected: Cow<'_, str>) -> Self;
}

impl ExpectedParseError for String {
    #[inline]
    fn expected(expected: Cow<'_, str>) -> Self {
        format!("expected {}", expected)
    }
}

impl ExpectedParseError for () {
    #[inline]
    fn expected(_: Cow<'_, str>) -> Self {}
}

impl<T> ExpectedParseError for DefaultParseError<T> {
    #[inline]
    fn expected(expected: Cow<'_, str>) -> Self {
        Self::Expected(expected.into_owned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericExpectation {
    I8,
    U8,
    NonZeroI8,
    NonZeroU8,

    BigEndianI16,
    LittleEndianI16,
    BigEndianU16,
    LittleEndianU16,
    BigEndianNonZeroI16,
    BigEndianNonZeroU16,
    LittleEndianNonZeroI16,
    LittleEndianNonZeroU16,

    BigEndianI32,
    LittleEndianI32,
    BigEndianU32,
    LittleEndianU32,
    BigEndianNonZeroI32,
    BigEndianNonZeroU32,
    LittleEndianNonZeroI32,
    LittleEndianNonZeroU32,

    BigEndianI64,
    LittleEndianI64,
    BigEndianU64,
    LittleEndianU64,
    BigEndianNonZeroI64,
    BigEndianNonZeroU64,
    LittleEndianNonZeroI64,
    LittleEndianNonZeroU64,

    BigEndianF32,
    LittleEndianF32,
    BigEndianNonZeroFloat,
    LittleEndianNonZeroFloat,

    BigEndianF64,
    LittleEndianF64,
    BigEndianNonZeroDouble,
    LittleEndianNonZeroDouble,
}

impl Display for NumericExpectation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::I8 => "signed 8-bit integer",
            Self::U8 => "unsigned 8-bit integer",
            Self::NonZeroI8 => "non-zero signed 8-bit integer",
            Self::NonZeroU8 => "non-zero unsigned 8-bit integer",
            Self::BigEndianI16 => "big-endian signed 16-bit integer",
            Self::LittleEndianI16 => "little-endian signed 16-bit integer",
            Self::BigEndianU16 => "big-endian unsigned 16-bit integer",
            Self::LittleEndianU16 => "little-endian unsigned 16-bit integer",
            Self::BigEndianNonZeroI16 => "big-endian non-zero signed 16-bit integer",
            Self::BigEndianNonZeroU16 => "big-endian non-zero unsigned 16-bit integer",
            Self::LittleEndianNonZeroI16 => "little-endian non-zero signed 16-bit integer",
            Self::LittleEndianNonZeroU16 => "little-endian non-zero unsigned 16-bit integer",
            Self::BigEndianI32 => "big-endian signed 32-bit integer",
            Self::LittleEndianI32 => "little-endian signed 32-bit integer",
            Self::BigEndianU32 => "big-endian unsigned 32-bit integer",
            Self::LittleEndianU32 => "little-endian unsigned 32-bit integer",
            Self::BigEndianNonZeroI32 => "big-endian non-zero signed 32-bit integer",
            Self::BigEndianNonZeroU32 => "big-endian non-zero unsigned 32-bit integer",
            Self::LittleEndianNonZeroI32 => "little-endian non-zero signed 32-bit integer",
            Self::LittleEndianNonZeroU32 => "little-endian non-zero unsigned 32-bit integer",
            Self::BigEndianI64 => "big-endian signed 64-bit integer",
            Self::LittleEndianI64 => "little-endian signed 64-bit integer",
            Self::BigEndianU64 => "big-endian unsigned 64-bit integer",
            Self::LittleEndianU64 => "little-endian unsigned 64-bit integer",
            Self::BigEndianNonZeroI64 => "big-endian non-zero signed 64-bit integer",
            Self::BigEndianNonZeroU64 => "big-endian non-zero unsigned 64-bit integer",
            Self::LittleEndianNonZeroI64 => "little-endian non-zero signed 64-bit integer",
            Self::LittleEndianNonZeroU64 => "little-endian non-zero unsigned 64-bit integer",
            Self::BigEndianF32 => "big-endian float",
            Self::LittleEndianF32 => "little-endian float",
            Self::BigEndianNonZeroFloat => "big-endian non-zero float",
            Self::LittleEndianNonZeroFloat => "little-endian non-zero float",
            Self::BigEndianF64 => "big-endian double",
            Self::LittleEndianF64 => "little-endian double",
            Self::BigEndianNonZeroDouble => "big-endian non-zero double",
            Self::LittleEndianNonZeroDouble => "little-endian non-zero double",
        };

        f.write_str(string)
    }
}

pub trait ExpectedNumericParseError {
    #[must_use]
    fn expected_numeric(expectation: NumericExpectation) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefaultParseError<T> {
    ExpectedEndOfInput,
    ExpectedString(String),
    ExpectedAnyToken,
    ExpectedToken(T),
    Expected(String),
    NumericExpectation(NumericExpectation),
}

impl<T: Display> Display for DefaultParseError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedEndOfInput => write!(f, "expected end of input"),
            Self::ExpectedString(string) => write!(f, r#"expected "{}""#, string),
            Self::ExpectedAnyToken => write!(f, "expected any character"),
            Self::ExpectedToken(character) => write!(f, "expected '{}'", character),
            Self::Expected(expectation) => write!(f, "expected {}", expectation),
            Self::NumericExpectation(expectation) => write!(f, "expected a {}", expectation),
        }
    }
}
