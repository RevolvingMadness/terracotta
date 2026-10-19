use crate::{
    input::Input,
    parser::Parser,
    result::{HardParseResult, OptionParseResult, ParseResult, ParseResultTExt},
};

pub trait ParsableDefinition<'input> {
    type Output;

    type Error;

    #[must_use]
    fn error(&self) -> Self::Error;
}

pub trait Parsable<'input, I: Input + ?Sized + 'input>: ParsableDefinition<'input> {
    fn parse(&self, parser: &mut Parser<'input, I, Self::Error>) -> ParseResult<Self::Output>;

    fn try_parse(
        &self,
        parser: &mut Parser<'input, I, Self::Error>,
    ) -> OptionParseResult<Self::Output> {
        self.parse(parser).into_option_parse_result()
    }

    fn expect_renamed_with(
        &self,
        parser: &mut Parser<'input, I, Self::Error>,
        error_fn: impl FnOnce() -> Self::Error,
    ) -> HardParseResult<Self::Output> {
        self.parse(parser)
            .into_hard_parse_result_with(parser, error_fn)
    }

    #[inline]
    fn expect_renamed(
        &self,
        parser: &mut Parser<'input, I, Self::Error>,
        error: Self::Error,
    ) -> HardParseResult<Self::Output> {
        self.expect_renamed_with(parser, || error)
    }

    #[inline]
    fn expect(&self, parser: &mut Parser<'input, I, Self::Error>) -> HardParseResult<Self::Output> {
        self.expect_renamed_with(parser, || self.error())
    }
}

pub trait ParsableWithContext<'input, I: Input + ?Sized + 'input, Context>:
    ParsableDefinition<'input>
{
    fn parse_with_context(
        &self,
        parser: &mut Parser<'input, I, Self::Error>,
        context: &mut Context,
    ) -> ParseResult<Self::Output>;

    fn try_parse_with_context(
        &self,
        parser: &mut Parser<'input, I, Self::Error>,
        context: &mut Context,
    ) -> OptionParseResult<Self::Output> {
        self.parse_with_context(parser, context)
            .into_option_parse_result()
    }

    fn expect_with_context_renamed_with(
        &self,
        parser: &mut Parser<'input, I, Self::Error>,
        context: &mut Context,
        error_fn: impl FnOnce() -> Self::Error,
    ) -> HardParseResult<Self::Output> {
        self.parse_with_context(parser, context)
            .into_hard_parse_result_with(parser, error_fn)
    }

    fn expect_with_context_renamed(
        &self,
        parser: &mut Parser<'input, I, Self::Error>,
        context: &mut Context,
        error: Self::Error,
    ) -> HardParseResult<Self::Output> {
        self.expect_with_context_renamed_with(parser, context, || error)
    }

    #[inline]
    fn expect_with_context(
        &self,
        parser: &mut Parser<'input, I, Self::Error>,
        context: &mut Context,
    ) -> HardParseResult<Self::Output> {
        self.expect_with_context_renamed_with(parser, context, || self.error())
    }
}

impl<'input, I: Input + ?Sized + 'input, Context, P: Parsable<'input, I>>
    ParsableWithContext<'input, I, Context> for P
{
    fn parse_with_context(
        &self,
        parser: &mut Parser<'input, I, Self::Error>,
        _: &mut Context,
    ) -> ParseResult<Self::Output> {
        self.parse(parser)
    }
}
