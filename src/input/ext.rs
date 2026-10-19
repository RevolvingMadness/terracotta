use alloc::vec::Vec;

use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableError, ParsableWithContext},
    parse_error::ExpectedEndOfInputParseError,
    parser::Parser,
    spanned::Spanned,
};

fn parse_fully<
    'input,
    I: Input + ?Sized,
    FullError: ExpectedEndOfInputParseError,
    R,
    ParseError,
>(
    input: &'input I,
    parse: impl FnOnce(&mut Parser<'input, I, FullError>) -> Result<R, ParseError>,
) -> Result<R, Vec<Spanned<FullError>>> {
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    if parser.has_no_errors() && !parser.is_at_end() {
        parser.add_error(FullError::expected_end_of_input());
    }

    let errors = parser.finish();

    result.map_err(|_| errors)
}

fn parse_fully_allow_trailing<'input, I: Input + ?Sized, FullError, R, ParseError>(
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
        E: ExpectedEndOfInputParseError,
        P: ParsableWithContext<'input, Self, E, Context> + ParsableError<E>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<P::Output, Vec<Spanned<E>>> {
        parse_fully(self, |parser| parsable.expect_with_context(parser, context))
    }

    #[inline]
    fn parse_fully_with_context_allow_trailing<
        'input,
        E,
        P: ParsableWithContext<'input, Self, E, Context> + ParsableError<E>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<P::Output, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| parsable.expect_with_context(parser, context))
    }

    #[inline]
    fn try_parse_fully_with_context<
        'input,
        E: ExpectedEndOfInputParseError,
        P: ParsableWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<Option<P::Output>, Vec<Spanned<E>>> {
        parse_fully(self, |parser| {
            parsable.try_parse_with_context(parser, context)
        })
    }

    #[inline]
    fn try_parse_fully_with_context_allow_trailing<
        'input,
        E,
        P: ParsableWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<Option<P::Output>, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| {
            parsable.try_parse_with_context(parser, context)
        })
    }

    #[inline]
    fn parse_fully<
        'input,
        E: ExpectedEndOfInputParseError,
        P: Parsable<'input, Self, E> + ParsableError<E>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output, Vec<Spanned<E>>> {
        parse_fully(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn parse_fully_allow_trailing<'input, E, P: Parsable<'input, Self, E> + ParsableError<E>>(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn try_parse_fully<'input, E: ExpectedEndOfInputParseError, P: Parsable<'input, Self, E>>(
        &'input self,
        parsable: &P,
    ) -> Result<Option<P::Output>, Vec<Spanned<E>>> {
        parse_fully(self, |parser| parsable.try_parse(parser))
    }

    #[inline]
    fn try_parse_fully_allow_trailing<'input, E, P: Parsable<'input, Self, E>>(
        &'input self,
        parsable: &P,
    ) -> Result<Option<P::Output>, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| parsable.try_parse(parser))
    }
}

impl<I: Input + ?Sized> InputExt for I {}
