use crate::failures::hard::HardParseFailure;

pub type HardParseResult<T> = Result<T, HardParseFailure>;
