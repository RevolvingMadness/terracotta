use crate::{
    failures::{ParseFailure, soft::SoftParseFailure},
    results::ParseResult,
};

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
