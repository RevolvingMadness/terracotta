use crate::{
    input::Input,
    parser::{CalledFromParser, Parser},
    result::{HardParseResult, OptionParseResult, ParseResult, ParseResultTExt},
};

pub trait Parsable<I: Input, Context> {
    const NAME: &'static str;

    type Output;

    fn parse(parser: &mut Parser<I, Context>, _: CalledFromParser) -> ParseResult<Self::Output>;

    fn try_parse(
        parser: &mut Parser<I, Context>,
        called_from_parser: CalledFromParser,
    ) -> OptionParseResult<Self::Output> {
        Self::parse(parser, called_from_parser).into_option_parse_result()
    }

    #[inline]
    fn expect(
        parser: &mut Parser<I, Context>,
        called_from_parser: CalledFromParser,
    ) -> HardParseResult<Self::Output> {
        Self::expect_named(parser, called_from_parser, Self::NAME)
    }

    fn expect_named(
        parser: &mut Parser<I, Context>,
        called_from_parser: CalledFromParser,
        expected: &str,
    ) -> HardParseResult<Self::Output> {
        Self::parse(parser, called_from_parser)
            .into_hard_parse_result(parser, || format!("expected {}", expected))
    }
}

pub trait ParsableInstance<I: Input, Context> {
    type Output;

    #[must_use]
    fn name(&self) -> String;

    fn parse_instance(&self, parser: &mut Parser<I, Context>) -> ParseResult<Self::Output>;

    fn try_parse_instance(
        &self,
        parser: &mut Parser<I, Context>,
    ) -> OptionParseResult<Self::Output> {
        self.parse_instance(parser).into_option_parse_result()
    }

    #[inline]
    fn expect_instance(&self, parser: &mut Parser<I, Context>) -> HardParseResult<Self::Output> {
        self.parse_instance(parser)
            .into_hard_parse_result(parser, || format!("expected {}", self.name()))
    }
}
