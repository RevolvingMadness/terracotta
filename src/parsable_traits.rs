use crate::{
    parser::{CalledFromParser, Parser},
    result::{HardParseResult, OptionParseResult, ParseResult, ParseResultTExt},
};

pub trait Parsable<'input, Context> {
    const NAME: &'static str;

    type Output;

    fn parse(
        parser: &mut Parser<'input, Context>,
        _: CalledFromParser,
    ) -> ParseResult<Self::Output>;

    fn try_parse(
        parser: &mut Parser<'input, Context>,
        key: CalledFromParser,
    ) -> OptionParseResult<Self::Output> {
        Self::parse(parser, key).into_option_parse_result()
    }

    #[inline]
    fn expect(
        parser: &mut Parser<'input, Context>,
        key: CalledFromParser,
    ) -> HardParseResult<Self::Output> {
        Self::expect_named(parser, key, Self::NAME)
    }

    fn expect_named(
        parser: &mut Parser<'input, Context>,
        key: CalledFromParser,
        expected: &str,
    ) -> HardParseResult<Self::Output> {
        Self::parse(parser, key).into_hard_parse_result(parser, || format!("expected {}", expected))
    }
}

pub trait ParsableInstance<'input, Context> {
    type Output;

    #[must_use]
    fn name(&self) -> String;

    fn parse_instance(&self, parser: &mut Parser<'input, Context>) -> ParseResult<Self::Output>;

    fn try_parse_instance(
        &self,
        parser: &mut Parser<'input, Context>,
    ) -> OptionParseResult<Self::Output> {
        self.parse_instance(parser).into_option_parse_result()
    }

    #[inline]
    fn expect_instance(
        &self,
        parser: &mut Parser<'input, Context>,
    ) -> HardParseResult<Self::Output> {
        self.parse_instance(parser)
            .into_hard_parse_result(parser, || format!("expected {}", self.name()))
    }
}
