use crate::{
    input::Input,
    parser::Parser,
    result::{HardParseResult, OptionParseResult, ParseResult, ParseResultTExt},
};

pub trait Parsable<I: Input> {
    const NAME: &'static str;

    type Output;

    fn parse(parser: &mut Parser<I>) -> ParseResult<Self::Output>;

    fn try_parse(parser: &mut Parser<I>) -> OptionParseResult<Self::Output> {
        Self::parse(parser).into_option_parse_result()
    }

    fn expect_renamed(parser: &mut Parser<I>, expected: &str) -> HardParseResult<Self::Output> {
        Self::parse(parser).into_hard_parse_result(parser, || format!("expected {}", expected))
    }

    #[inline]
    fn expect(parser: &mut Parser<I>) -> HardParseResult<Self::Output> {
        Self::expect_renamed(parser, Self::NAME)
    }
}

pub trait ParsableWithContext<I: Input, Context> {
    const NAME: &'static str;

    type Output;

    fn parse_with_context(parser: &mut Parser<I>, context: &Context) -> ParseResult<Self::Output>;

    fn try_parse_with_context(
        parser: &mut Parser<I>,
        context: &Context,
    ) -> OptionParseResult<Self::Output> {
        Self::parse_with_context(parser, context).into_option_parse_result()
    }

    fn expect_with_context_renamed(
        parser: &mut Parser<I>,
        context: &Context,
        expected: &str,
    ) -> HardParseResult<Self::Output> {
        Self::parse_with_context(parser, context)
            .into_hard_parse_result(parser, || format!("expected {}", expected))
    }

    #[inline]
    fn expect_with_context(
        parser: &mut Parser<I>,
        context: &Context,
    ) -> HardParseResult<Self::Output> {
        Self::expect_with_context_renamed(parser, context, Self::NAME)
    }
}

impl<I: Input, Context, P: Parsable<I>> ParsableWithContext<I, Context> for P {
    const NAME: &'static str = P::NAME;

    type Output = P::Output;

    fn parse_with_context(parser: &mut Parser<I>, _: &Context) -> ParseResult<Self::Output> {
        P::parse(parser)
    }
}

pub trait ParsableInstance<I: Input> {
    type Output;

    #[must_use]
    fn name(&self) -> String;

    fn instance_parse(&self, parser: &mut Parser<I>) -> ParseResult<Self::Output>;

    fn instance_try_parse(&self, parser: &mut Parser<I>) -> OptionParseResult<Self::Output> {
        self.instance_parse(parser).into_option_parse_result()
    }

    #[inline]
    fn instance_expect(&self, parser: &mut Parser<I>) -> HardParseResult<Self::Output> {
        self.instance_parse(parser)
            .into_hard_parse_result(parser, || format!("expected {}", self.name()))
    }
}

pub trait ParsableInstanceWithContext<I: Input, Context> {
    type Output;

    #[must_use]
    fn name(&self) -> String;

    fn instance_parse_with_context(
        &self,
        parser: &mut Parser<I>,
        context: &Context,
    ) -> ParseResult<Self::Output>;

    fn instance_try_parse_with_context(
        &self,
        parser: &mut Parser<I>,
        context: &Context,
    ) -> OptionParseResult<Self::Output> {
        self.instance_parse_with_context(parser, context)
            .into_option_parse_result()
    }

    #[inline]
    fn instance_expect_with_context(
        &self,
        parser: &mut Parser<I>,
        context: &Context,
    ) -> HardParseResult<Self::Output> {
        self.instance_parse_with_context(parser, context)
            .into_hard_parse_result(parser, || format!("expected {}", self.name()))
    }
}

impl<I: Input, Context, P: ParsableInstance<I>> ParsableInstanceWithContext<I, Context> for P {
    type Output = P::Output;

    fn name(&self) -> String {
        self.name()
    }

    fn instance_parse_with_context(
        &self,
        parser: &mut Parser<I>,
        _: &Context,
    ) -> ParseResult<Self::Output> {
        self.instance_parse(parser)
    }
}
