use alloc::vec::Vec;

use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableDefinition, ParsableWithContext},
    parse_error::ExpectedEndOfInputParseError,
    parser::Parser,
    spanned::Spanned,
};

fn parse_fully<
    'input,
    I: Input + ?Sized + 'input,
    FullError: From<ExpectedEndOfInputParseError>,
    R,
    ParseError,
>(
    input: &'input I,
    parse: impl FnOnce(&mut Parser<'input, I, FullError>) -> Result<R, ParseError>,
) -> Result<R, Vec<Spanned<FullError>>> {
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    if parser.has_no_errors() && !parser.is_at_end() {
        parser.add_error(FullError::from(ExpectedEndOfInputParseError));
    }

    let errors = parser.finish();

    result.map_err(|_| errors)
}

fn parse_fully_allow_trailing<'input, I: Input + ?Sized + 'input, FullError, R, ParseError>(
    input: &'input I,
    parse: impl FnOnce(&mut Parser<'input, I, FullError>) -> Result<R, ParseError>,
) -> Result<R, Vec<Spanned<FullError>>> {
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    let errors = parser.finish();

    result.map_err(|_| errors)
}

pub trait InputExt: Input {
    #[inline]
    fn parse_fully_with_context<
        'input,
        P: ParsableDefinition<'input> + ParsableWithContext<'input, Self, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<P::Output, Vec<Spanned<P::Error>>>
    where
        P::Error: From<ExpectedEndOfInputParseError>,
    {
        parse_fully(self, |parser| parsable.expect_with_context(parser, context))
    }

    #[inline]
    fn parse_fully_with_context_allow_trailing<
        'input,
        P: ParsableDefinition<'input> + ParsableWithContext<'input, Self, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<P::Output, Vec<Spanned<P::Error>>> {
        parse_fully_allow_trailing(self, |parser| parsable.expect_with_context(parser, context))
    }

    #[inline]
    fn try_parse_fully_with_context<
        'input,
        P: ParsableWithContext<'input, Self, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<Option<P::Output>, Vec<Spanned<P::Error>>>
    where
        P::Error: From<ExpectedEndOfInputParseError>,
    {
        parse_fully(self, |parser| {
            parsable.try_parse_with_context(parser, context)
        })
    }

    #[inline]
    fn try_parse_fully_with_context_allow_trailing<
        'input,
        P: ParsableWithContext<'input, Self, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<Option<P::Output>, Vec<Spanned<P::Error>>> {
        parse_fully_allow_trailing(self, |parser| {
            parsable.try_parse_with_context(parser, context)
        })
    }

    #[inline]
    fn parse_fully<'input, P: Parsable<'input, Self> + ParsableDefinition<'input>>(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output, Vec<Spanned<P::Error>>>
    where
        P::Error: From<ExpectedEndOfInputParseError>,
    {
        parse_fully(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn parse_fully_allow_trailing<
        'input,
        P: Parsable<'input, Self> + ParsableDefinition<'input>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output, Vec<Spanned<P::Error>>> {
        parse_fully_allow_trailing(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn try_parse_fully<'input, P: Parsable<'input, Self>>(
        &'input self,
        parsable: &P,
    ) -> Result<Option<P::Output>, Vec<Spanned<P::Error>>>
    where
        P::Error: From<ExpectedEndOfInputParseError>,
    {
        parse_fully(self, |parser| parsable.try_parse(parser))
    }

    #[inline]
    fn try_parse_fully_allow_trailing<'input, P: Parsable<'input, Self>>(
        &'input self,
        parsable: &P,
    ) -> Result<Option<P::Output>, Vec<Spanned<P::Error>>> {
        parse_fully_allow_trailing(self, |parser| parsable.try_parse(parser))
    }
}

impl<I: Input + ?Sized> InputExt for I {}
