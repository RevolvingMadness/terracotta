use crate::{
    input::Input,
    parser::Parser,
    results::{ParseResult, ParseResultTExt, hard::HardParseResult, option::OptionParseResult},
    traits::parsable_definition::ParsableDefinition,
};

pub trait Parsable<'input, I, E>: ParsableDefinition
where
    I: Input + ?Sized + 'input,
{
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output<'input, I>>;

    fn try_parse(
        &self,
        parser: &mut Parser<'input, I, E>,
    ) -> OptionParseResult<Self::Output<'input, I>> {
        self.parse(parser).into_option_parse_result()
    }

    fn expect_renamed_with<E2: Into<E>>(
        &self,
        parser: &mut Parser<'input, I, E>,
        error_fn: impl FnOnce() -> E2,
    ) -> HardParseResult<Self::Output<'input, I>> {
        self.parse(parser)
            .into_hard_parse_result_with(parser, error_fn)
    }

    #[inline]
    fn expect_renamed<E2: Into<E>>(
        &self,
        parser: &mut Parser<'input, I, E>,
        error: E2,
    ) -> HardParseResult<Self::Output<'input, I>> {
        self.expect_renamed_with(parser, || error)
    }

    #[inline]
    fn expect(&self, parser: &mut Parser<'input, I, E>) -> HardParseResult<Self::Output<'input, I>>
    where
        E: From<Self::Error>,
    {
        self.expect_renamed_with(parser, || self.error())
    }
}
