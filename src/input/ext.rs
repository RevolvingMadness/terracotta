use alloc::vec::Vec;

use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableDefinition, ParsableWithContext},
    parse_error::ExpectedEndOfInputParseError,
    parser::Parser,
    spanned::Spanned,
};

fn parse_fully<'input, I, ParserError, Output, ParsableError>(
    input: &'input I,
    parse: impl FnOnce(&mut Parser<'input, I, ParserError>) -> Result<Output, ParsableError>,
) -> Result<Output, Vec<Spanned<ParserError>>>
where
    I: Input + ?Sized + 'input,
    ParserError: From<ExpectedEndOfInputParseError>,
{
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    if parser.has_no_errors() && !parser.is_at_end() {
        parser.add_error(ParserError::from(ExpectedEndOfInputParseError));
    }

    let errors = parser.finish();

    result.map_err(|_| errors)
}

fn parse_fully_allow_trailing<'input, I, ParserError, Output, ParsableError>(
    input: &'input I,
    parse: impl FnOnce(&mut Parser<'input, I, ParserError>) -> Result<Output, ParsableError>,
) -> Result<Output, Vec<Spanned<ParserError>>>
where
    I: Input + ?Sized + 'input,
{
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    let errors = parser.finish();

    result.map_err(|_| errors)
}

pub trait InputExt<'input>: Input {
    #[inline]
    fn parse_fully_with_context<
        E: From<ExpectedEndOfInputParseError>,
        P: ParsableDefinition<'input, Error = E> + ParsableWithContext<'input, Self, E, Context>,
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
        E,
        P: ParsableDefinition<'input, Error = E> + ParsableWithContext<'input, Self, E, Context>,
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
        E: From<ExpectedEndOfInputParseError>,
        P: ParsableDefinition<'input, Error = E> + ParsableWithContext<'input, Self, E, Context>,
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
        E,
        P: ParsableDefinition<'input, Error = E> + ParsableWithContext<'input, Self, E, Context>,
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
        E: From<ExpectedEndOfInputParseError>,
        P: ParsableDefinition<'input, Error = E> + Parsable<'input, Self, E>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output, Vec<Spanned<P::Error>>> {
        parse_fully(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn parse_fully_allow_trailing<
        E,
        P: ParsableDefinition<'input, Error = E> + Parsable<'input, Self, E>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn try_parse_fully<
        E: From<ExpectedEndOfInputParseError>,
        P: ParsableDefinition<'input, Error = E> + Parsable<'input, Self, E>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<Option<P::Output>, Vec<Spanned<E>>> {
        parse_fully(self, |parser| parsable.try_parse(parser))
    }

    #[inline]
    fn try_parse_fully_allow_trailing<E, P: Parsable<'input, Self, E>>(
        &'input self,
        parsable: &P,
    ) -> Result<Option<P::Output>, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| parsable.try_parse(parser))
    }
}

impl<'input, I> InputExt<'input> for I where I: Input + ?Sized + 'input {}
