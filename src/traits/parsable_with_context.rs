use crate::{
    input::Input,
    parser::Parser,
    results::{ParseResult, ParseResultTExt, hard::HardParseResult, option::OptionParseResult},
    traits::parsable_definition::ParsableDefinition,
};

pub trait ParsableWithContext<'input, I, E, C>: ParsableDefinition
where
    I: Input + ?Sized + 'input,
{
    fn parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut C,
    ) -> ParseResult<Self::Output<'input>>;

    fn try_parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut C,
    ) -> OptionParseResult<Self::Output<'input>> {
        self.parse_with_context(parser, context)
            .into_option_parse_result()
    }

    fn expect_with_context_renamed_with(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut C,
        error_fn: impl FnOnce() -> Self::Error,
    ) -> HardParseResult<Self::Output<'input>>
    where
        E: From<Self::Error>,
    {
        self.parse_with_context(parser, context)
            .into_hard_parse_result_with(parser, error_fn)
    }

    fn expect_with_context_renamed(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut C,
        error: Self::Error,
    ) -> HardParseResult<Self::Output<'input>>
    where
        E: From<Self::Error>,
    {
        self.expect_with_context_renamed_with(parser, context, || error)
    }

    #[inline]
    fn expect_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut C,
    ) -> HardParseResult<Self::Output<'input>>
    where
        E: From<Self::Error>,
    {
        self.expect_with_context_renamed_with(parser, context, || self.error())
    }
}
