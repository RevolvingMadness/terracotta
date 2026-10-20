use crate::{
    failures::{ParseFailure, hard::HardParseFailure, soft::SoftParseFailure},
    results::{ParseResult, soft::SoftParseResult},
};

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

    fn into_parse_result(self) -> ParseResult<T>;
}

impl<T> OptionTExt<T> for Option<T> {
    fn into_soft_parse_result(self) -> SoftParseResult<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(SoftParseFailure),
        }
    }

    #[inline]
    fn into_parse_result(self) -> ParseResult<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(ParseFailure::Soft),
        }
    }
}
