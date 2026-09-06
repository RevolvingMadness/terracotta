use crate::{input::Input, parser::Parser};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoftParseFailure;

impl From<SoftParseFailure> for ParseFailure {
    fn from(SoftParseFailure: SoftParseFailure) -> Self {
        Self::Soft
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HardParseFailure(pub(crate) ());

impl From<HardParseFailure> for ParseFailure {
    fn from(value: HardParseFailure) -> Self {
        Self::Hard(value)
    }
}

impl HardParseFailure {
    #[inline]
    pub const fn into_parse_result<T>(self) -> ParseResult<T> {
        Err(ParseFailure::Hard(self))
    }

    #[inline]
    pub const fn into_hard_parse_result<T>(self) -> HardParseResult<T> {
        Err(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseFailure {
    Soft,
    Hard(HardParseFailure),
}

pub type ParseResult<T> = Result<T, ParseFailure>;

pub trait ParseResultTExt<T> {
    fn into_option_parse_result(self) -> OptionParseResult<T>;

    fn into_hard_parse_result_with<I: Input + ?Sized, E>(
        self,
        parser: &mut Parser<I, E>,
        error_fn: impl FnOnce() -> E,
    ) -> HardParseResult<T>;

    fn into_hard_parse_result<I: Input + ?Sized, E>(
        self,
        parser: &mut Parser<I, E>,
        error: E,
    ) -> HardParseResult<T>;
}

impl<T> ParseResultTExt<T> for ParseResult<T> {
    fn into_option_parse_result(self) -> OptionParseResult<T> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(ParseFailure::Soft) => Ok(None),
            Err(ParseFailure::Hard(failure)) => Err(failure),
        }
    }

    fn into_hard_parse_result_with<I: Input + ?Sized, E>(
        self,
        parser: &mut Parser<I, E>,
        error_fn: impl FnOnce() -> E,
    ) -> HardParseResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(ParseFailure::Soft) => {
                let error = error_fn();

                let failure = parser.add_error(error);

                Err(failure)
            }
            Err(ParseFailure::Hard(failure)) => Err(failure),
        }
    }

    #[inline]
    fn into_hard_parse_result<I: Input + ?Sized, E>(
        self,
        parser: &mut Parser<I, E>,
        error: E,
    ) -> HardParseResult<T> {
        self.into_hard_parse_result_with(parser, || error)
    }
}

pub type SoftParseResult<T> = Result<T, SoftParseFailure>;

pub trait SoftParseResultTExt<T> {
    fn into_parse_result(self) -> ParseResult<T>;
}

impl<T> SoftParseResultTExt<T> for SoftParseResult<T> {
    fn into_parse_result(self) -> ParseResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(SoftParseFailure) => Err(ParseFailure::Soft),
        }
    }
}

pub type HardParseResult<T> = Result<T, HardParseFailure>;

pub type OptionParseResult<T> = Result<Option<T>, HardParseFailure>;

pub trait OptionParseResultTExt<T> {
    fn into_parse_result(self) -> ParseResult<T>;
}

impl<T> OptionParseResultTExt<T> for OptionParseResult<T> {
    fn into_parse_result(self) -> ParseResult<T> {
        match self {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Err(ParseFailure::Soft),
            Err(failure) => Err(ParseFailure::Hard(failure)),
        }
    }
}

pub trait OptionTExt<T> {
    fn into_soft_parse_result(self) -> SoftParseResult<T>;

    fn into_parse_result_soft(self) -> ParseResult<T>;
}

impl<T> OptionTExt<T> for Option<T> {
    fn into_soft_parse_result(self) -> SoftParseResult<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(SoftParseFailure),
        }
    }

    #[inline]
    fn into_parse_result_soft(self) -> ParseResult<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(ParseFailure::Soft),
        }
    }
}
