use crate::parser::Parser;

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
    #[must_use]
    pub const fn new_unchecked() -> Self {
        Self(())
    }
}

pub enum ParseFailure {
    Soft,
    Hard(HardParseFailure),
}

pub type ParseResult<T> = Result<T, ParseFailure>;

pub trait ParseResultTExt<T> {
    fn into_option_parse_result(self) -> OptionParseResult<T>;

    fn into_hard_parse_result<Context>(
        self,
        parser: &mut Parser<'_, Context>,
        message: impl FnOnce() -> String,
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

    fn into_hard_parse_result<Context>(
        self,
        parser: &mut Parser<'_, Context>,
        message: impl FnOnce() -> String,
    ) -> HardParseResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(ParseFailure::Soft) => {
                let message = message();

                let failure = parser.add_error_with_length_one(message);

                Err(failure)
            }
            Err(ParseFailure::Hard(failure)) => Err(failure),
        }
    }
}

pub type SoftParseResult<T> = Result<T, SoftParseFailure>;

pub type HardParseResult<T> = Result<T, HardParseFailure>;

pub type OptionParseResult<T> = Result<Option<T>, HardParseFailure>;
