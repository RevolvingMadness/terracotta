use crate::{
    input::Input,
    parser::Parser,
    result::{HardParseResult, OptionParseResult, ParseResult, ParseResultTExt},
};

pub trait ParsableError<E> {
    #[must_use]
    fn error() -> E;
}

pub trait Parsable<'input, I: Input + ?Sized, E> {
    type Output;

    fn parse(parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output>;

    fn try_parse(parser: &mut Parser<'input, I, E>) -> OptionParseResult<Self::Output> {
        Self::parse(parser).into_option_parse_result()
    }

    fn expect_renamed(
        parser: &mut Parser<'input, I, E>,
        error_fn: impl FnOnce() -> E,
    ) -> HardParseResult<Self::Output> {
        Self::parse(parser).into_hard_parse_result(parser, error_fn)
    }

    #[inline]
    fn expect(parser: &mut Parser<'input, I, E>) -> HardParseResult<Self::Output>
    where
        Self: ParsableError<E>,
    {
        Self::parse(parser).into_hard_parse_result(parser, Self::error)
    }
}

pub trait ParsableWithContext<'input, I: Input + ?Sized, E, Context> {
    type Output;

    fn parse_with_context(
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
    ) -> ParseResult<Self::Output>;

    fn try_parse_with_context(
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
    ) -> OptionParseResult<Self::Output> {
        Self::parse_with_context(parser, context).into_option_parse_result()
    }

    fn expect_with_context_renamed(
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
        error_fn: impl FnOnce() -> E,
    ) -> HardParseResult<Self::Output> {
        Self::parse_with_context(parser, context).into_hard_parse_result(parser, error_fn)
    }

    #[inline]
    fn expect_with_context(
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
    ) -> HardParseResult<Self::Output>
    where
        Self: ParsableError<E>,
    {
        Self::expect_with_context_renamed(parser, context, Self::error)
    }
}

impl<'input, I: Input + ?Sized, E, Context, P: Parsable<'input, I, E>>
    ParsableWithContext<'input, I, E, Context> for P
{
    type Output = P::Output;

    fn parse_with_context(
        parser: &mut Parser<'input, I, E>,
        _: &mut Context,
    ) -> ParseResult<Self::Output> {
        P::parse(parser)
    }
}

pub trait ParsableInstanceError<E> {
    #[must_use]
    fn error(&self) -> E;
}

pub trait ParsableInstance<'input, I: Input + ?Sized, E> {
    type Output;

    fn instance_parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output>;

    fn instance_try_parse(
        &self,
        parser: &mut Parser<'input, I, E>,
    ) -> OptionParseResult<Self::Output> {
        self.instance_parse(parser).into_option_parse_result()
    }

    #[inline]
    fn instance_expect(&self, parser: &mut Parser<'input, I, E>) -> HardParseResult<Self::Output>
    where
        Self: ParsableInstanceError<E>,
    {
        self.instance_parse(parser)
            .into_hard_parse_result(parser, || self.error())
    }
}

pub trait ParsableInstanceWithContext<'input, I: Input + ?Sized, E, Context> {
    type Output;

    fn instance_parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
    ) -> ParseResult<Self::Output>;

    fn instance_try_parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
    ) -> OptionParseResult<Self::Output> {
        self.instance_parse_with_context(parser, context)
            .into_option_parse_result()
    }

    #[inline]
    fn instance_expect_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut Context,
    ) -> HardParseResult<Self::Output>
    where
        Self: ParsableInstanceError<E>,
    {
        self.instance_parse_with_context(parser, context)
            .into_hard_parse_result(parser, || self.error())
    }
}

impl<'input, I: Input + ?Sized, E, Context, P: ParsableInstance<'input, I, E>>
    ParsableInstanceWithContext<'input, I, E, Context> for P
{
    type Output = P::Output;

    fn instance_parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        _: &mut Context,
    ) -> ParseResult<Self::Output> {
        self.instance_parse(parser)
    }
}
