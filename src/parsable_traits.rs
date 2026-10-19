use crate::{
    input::Input,
    parser::Parser,
    result::{HardParseResult, OptionParseResult, ParseResult, ParseResultTExt},
};

pub trait ParsableError<E> {
    #[must_use]
    fn error(&self) -> E;
}

pub trait Parsable<'input, I: Input + ?Sized, E> {
    type Output;

    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output>;

    fn try_parse(&self, parser: &mut Parser<'input, I, E>) -> OptionParseResult<Self::Output> {
        self.parse(parser).into_option_parse_result()
    }

    fn expect_renamed_with(
        &self,
        parser: &mut Parser<'input, I, E>,
        error_fn: impl FnOnce() -> E,
    ) -> HardParseResult<Self::Output> {
        self.parse(parser)
            .into_hard_parse_result_with(parser, error_fn)
    }

    #[inline]
    fn expect_renamed(
        &self,
        parser: &mut Parser<'input, I, E>,
        error: E,
    ) -> HardParseResult<Self::Output> {
        self.expect_renamed_with(parser, || error)
    }

    #[inline]
    fn expect(&self, parser: &mut Parser<'input, I, E>) -> HardParseResult<Self::Output>
    where
        Self: ParsableError<E>,
    {
        self.expect_renamed_with(parser, || self.error())
    }
}

pub trait ParsableWithContext<'input, I: Input + ?Sized, E, Context> {
    type Output;

    fn parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
    ) -> ParseResult<Self::Output>;

    fn try_parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
    ) -> OptionParseResult<Self::Output> {
        self.parse_with_context(parser, context)
            .into_option_parse_result()
    }

    fn expect_with_context_renamed_with(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
        error_fn: impl FnOnce() -> E,
    ) -> HardParseResult<Self::Output> {
        self.parse_with_context(parser, context)
            .into_hard_parse_result_with(parser, error_fn)
    }

    fn expect_with_context_renamed(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
        error: E,
    ) -> HardParseResult<Self::Output> {
        self.expect_with_context_renamed_with(parser, context, || error)
    }

    #[inline]
    fn expect_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
    ) -> HardParseResult<Self::Output>
    where
        Self: ParsableError<E>,
    {
        self.expect_with_context_renamed_with(parser, context, || self.error())
    }
}

impl<'input, I: Input + ?Sized, E, Context, P: Parsable<'input, I, E>>
    ParsableWithContext<'input, I, E, Context> for P
{
    type Output = P::Output;

    fn parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        _: &mut Context,
    ) -> ParseResult<Self::Output> {
        self.parse(parser)
    }
}
