use crate::{
    input::Input,
    parser::Parser,
    result::{HardParseResult, OptionParseResult, ParseResult, ParseResultTExt},
};

pub trait Parsable<'input, I: Input + ?Sized> {
    const NAME: &'static str;

    type Output;

    fn parse(parser: &mut Parser<'input, I>) -> ParseResult<Self::Output>;

    fn try_parse(parser: &mut Parser<'input, I>) -> OptionParseResult<Self::Output> {
        Self::parse(parser).into_option_parse_result()
    }

    fn expect_renamed(
        parser: &mut Parser<'input, I>,
        expected: &str,
    ) -> HardParseResult<Self::Output> {
        Self::parse(parser).into_hard_parse_result(parser, || format!("expected {}", expected))
    }

    #[inline]
    fn expect(parser: &mut Parser<'input, I>) -> HardParseResult<Self::Output> {
        Self::expect_renamed(parser, Self::NAME)
    }
}

pub trait ParsableWithContext<'input, I: Input + ?Sized, Context> {
    const NAME: &'static str;

    type Output;

    fn parse_with_context(
        parser: &mut Parser<'input, I>,
        context: &mut Context,
    ) -> ParseResult<Self::Output>;

    fn try_parse_with_context(
        parser: &mut Parser<'input, I>,
        context: &mut Context,
    ) -> OptionParseResult<Self::Output> {
        Self::parse_with_context(parser, context).into_option_parse_result()
    }

    fn expect_with_context_renamed(
        parser: &mut Parser<'input, I>,
        context: &mut Context,
        expected: &str,
    ) -> HardParseResult<Self::Output> {
        Self::parse_with_context(parser, context)
            .into_hard_parse_result(parser, || format!("expected {}", expected))
    }

    #[inline]
    fn expect_with_context(
        parser: &mut Parser<'input, I>,
        context: &mut Context,
    ) -> HardParseResult<Self::Output> {
        Self::expect_with_context_renamed(parser, context, Self::NAME)
    }
}

impl<'input, I: Input + ?Sized, Context, P: Parsable<'input, I>>
    ParsableWithContext<'input, I, Context> for P
{
    const NAME: &'static str = P::NAME;

    type Output = P::Output;

    fn parse_with_context(
        parser: &mut Parser<'input, I>,
        _: &mut Context,
    ) -> ParseResult<Self::Output> {
        P::parse(parser)
    }
}

pub trait ParsableInstance<'input, I: Input + ?Sized> {
    type Output;

    #[must_use]
    fn name(&self) -> String;

    fn instance_parse(&self, parser: &mut Parser<'input, I>) -> ParseResult<Self::Output>;

    fn instance_try_parse(
        &self,
        parser: &mut Parser<'input, I>,
    ) -> OptionParseResult<Self::Output> {
        self.instance_parse(parser).into_option_parse_result()
    }

    #[inline]
    fn instance_expect(&self, parser: &mut Parser<'input, I>) -> HardParseResult<Self::Output> {
        self.instance_parse(parser)
            .into_hard_parse_result(parser, || format!("expected {}", self.name()))
    }
}

pub trait ParsableInstanceWithContext<'input, I: Input + ?Sized, Context> {
    type Output;

    #[must_use]
    fn name(&self) -> String;

    fn instance_parse_with_context(
        &self,
        parser: &mut Parser<'input, I>,
        context: &mut Context,
    ) -> ParseResult<Self::Output>;

    fn instance_try_parse_with_context(
        &self,
        parser: &mut Parser<'input, I>,
        context: &mut Context,
    ) -> OptionParseResult<Self::Output> {
        self.instance_parse_with_context(parser, context)
            .into_option_parse_result()
    }

    #[inline]
    fn instance_expect_with_context(
        &self,
        parser: &mut Parser<'input, I>,
        context: &mut Context,
    ) -> HardParseResult<Self::Output> {
        self.instance_parse_with_context(parser, context)
            .into_hard_parse_result(parser, || format!("expected {}", self.name()))
    }
}

impl<'input, I: Input + ?Sized, Context, P: ParsableInstance<'input, I>>
    ParsableInstanceWithContext<'input, I, Context> for P
{
    type Output = P::Output;

    fn name(&self) -> String {
        self.name()
    }

    fn instance_parse_with_context(
        &self,
        parser: &mut Parser<'input, I>,
        _: &mut Context,
    ) -> ParseResult<Self::Output> {
        self.instance_parse(parser)
    }
}
