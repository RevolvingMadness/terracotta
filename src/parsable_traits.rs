use crate::{
    parser::Parser,
    result::{hard::HardParseResult, regular::ParseResult},
};

pub trait Parsable<'input, Context>: Sized {
    const NAME: &'static str;

    type Output;

    fn parse(parser: &mut Parser<'input, Context>) -> ParseResult<Self::Output>;

    fn try_parse(parser: &mut Parser<'input, Context>) -> HardParseResult<Option<Self::Output>> {
        Self::parse(parser).into_hard_option_parse_result()
    }

    #[inline]
    fn expect(parser: &mut Parser<'input, Context>) -> HardParseResult<Self::Output> {
        Self::expect_named(parser, || Self::NAME.to_owned())
    }

    fn expect_named(
        parser: &mut Parser<'input, Context>,
        expected: impl FnOnce() -> String,
    ) -> HardParseResult<Self::Output> {
        Self::parse(parser).into_hard_parse_result(parser, expected)
    }
}

pub trait ParsableInstance<'input, Context>: Sized {
    type Output;

    #[must_use]
    fn name(&self) -> String;

    fn parse_instance(&self, parser: &mut Parser<'input, Context>) -> ParseResult<Self::Output>;

    #[inline]
    fn expect_instance(
        self,
        parser: &mut Parser<'input, Context>,
    ) -> HardParseResult<Self::Output> {
        self.parse_instance(parser)
            .into_hard_parse_result(parser, || self.name())
    }
}
