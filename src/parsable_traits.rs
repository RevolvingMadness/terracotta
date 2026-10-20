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

pub trait Parsable<'input, I, E>: ParsableDefinition<'input>
where
    I: Input + ?Sized + 'input,
{
    fn parse(&self, parser: &mut Parser<'input, I, E>) -> ParseResult<Self::Output>;

    fn try_parse(&self, parser: &mut Parser<'input, I, E>) -> OptionParseResult<Self::Output> {
        self.parse(parser).into_option_parse_result()
    }

    fn expect_renamed_with<E2: Into<E>>(
        &self,
        parser: &mut Parser<'input, I, E>,
        error_fn: impl FnOnce() -> E2,
    ) -> HardParseResult<Self::Output> {
        self.parse(parser)
            .into_hard_parse_result_with(parser, error_fn)
    }

    #[inline]
    fn expect_renamed<E2: Into<E>>(
        &self,
        parser: &mut Parser<'input, I, E>,
        error: E2,
    ) -> HardParseResult<Self::Output> {
        self.expect_renamed_with(parser, || error)
    }

    #[inline]
    fn expect(&self, parser: &mut Parser<'input, I, E>) -> HardParseResult<Self::Output>
    where
        E: From<Self::Error>,
    {
        self.expect_renamed_with(parser, || self.error())
    }
}

pub trait ParsableWithContext<'input, I, E, C>: ParsableDefinition<'input>
where
    I: Input + ?Sized + 'input,
{
    fn parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut C,
    ) -> ParseResult<Self::Output>;

    fn try_parse_with_context(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut C,
    ) -> OptionParseResult<Self::Output> {
        self.parse_with_context(parser, context)
            .into_option_parse_result()
    }

    fn expect_with_context_renamed_with(
        &self,
        parser: &mut Parser<'input, I, E>,
        context: &mut C,
        error_fn: impl FnOnce() -> Self::Error,
    ) -> HardParseResult<Self::Output>
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
    ) -> HardParseResult<Self::Output>
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
    ) -> HardParseResult<Self::Output>
    where
        E: From<Self::Error>,
    {
        self.expect_with_context_renamed_with(parser, context, || self.error())
    }
}
//
// impl<'input, I, E, Context, P> ParsableWithContext<'input, I, E, Context> for P
// where
//     I: Input + ?Sized + 'input,
//     E,
//     P: Parsable<'input, I, E>,
// {
//     fn parse_with_context(
//         &self,
//         parser: &mut Parser<'input, I, E>,
//         _: &mut Context,
//     ) -> ParseResult<Self::Output> {
//         self.parse(parser)
//     }
// }
