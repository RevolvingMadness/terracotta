use crate::{
    failures::ParseFailure,
    input::Input,
    parser::Parser,
    results::{hard::HardParseResult, option::OptionParseResult},
};

pub mod hard;
pub mod option;
pub mod soft;

pub type ParseResult<T> = Result<T, ParseFailure>;

pub trait ParseResultTExt<T> {
    fn into_option_parse_result(self) -> OptionParseResult<T>;

    fn into_hard_parse_result_with<'input, I: Input + ?Sized + 'input, E, E2: Into<E>>(
        self,
        parser: &mut Parser<'input, I, E>,
        error_fn: impl FnOnce() -> E2,
    ) -> HardParseResult<T>;

    fn into_hard_parse_result<'input, I: Input + ?Sized + 'input, E, E2: Into<E>>(
        self,
        parser: &mut Parser<'input, I, E>,
        error: E2,
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

    fn into_hard_parse_result_with<'input, I: Input + ?Sized + 'input, E, E2: Into<E>>(
        self,
        parser: &mut Parser<'input, I, E>,
        error_fn: impl FnOnce() -> E2,
    ) -> HardParseResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(ParseFailure::Soft) => {
                let error = error_fn();

                let error = error.into();

                let failure = parser.add_error(error);

                Err(failure)
            }
            Err(ParseFailure::Hard(failure)) => Err(failure),
        }
    }

    #[inline]
    fn into_hard_parse_result<'input, I: Input + ?Sized + 'input, E, E2: Into<E>>(
        self,
        parser: &mut Parser<'input, I, E>,
        error: E2,
    ) -> HardParseResult<T> {
        self.into_hard_parse_result_with(parser, || error)
    }
}
