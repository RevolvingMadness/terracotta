use crate::{
    failures::ParseFailure,
    results::{ParseResult, hard::HardParseResult},
};

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
