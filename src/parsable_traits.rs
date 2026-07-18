use crate::{
    parser::Parser,
    result::{hard::HardParseResult, regular::ParseResult},
};

pub trait Parsable<'input, Context>: Sized {
    const NAME: &'static str;

    fn parse(parser: &mut Parser<'input, Context>) -> ParseResult<Self>;

    #[inline]
    fn expect(parser: &mut Parser<'input, Context>) -> HardParseResult<Self> {
        Self::expect_named(parser, || Self::NAME.to_owned())
    }

    fn expect_named(
        parser: &mut Parser<'input, Context>,
        expected: impl FnOnce() -> String,
    ) -> HardParseResult<Self> {
        Self::parse(parser).into_hard_parse_result(parser, expected)
    }
}

pub trait ParsableInstance<'name, 'input, Context>: Sized {
    #[must_use]
    fn name(&self) -> String;

    fn parse_instance(&self, parser: &mut Parser<'input, Context>) -> ParseResult<()>;

    #[inline]
    fn expect_instance(&self, parser: &mut Parser<'input, Context>) -> HardParseResult<()> {
        self.parse_instance(parser)
            .into_hard_parse_result(parser, || self.name())
    }
}
