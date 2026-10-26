use alloc::vec::Vec;

use crate::{
    errors::end_of_input::MaybeExpectedEndOfInputParseError,
    input::Input,
    parser::Parser,
    spanned::Spanned,
    traits::{parsable::Parsable, parsable_with_context::ParsableWithContext},
};

pub type ParseFullyError<E> = Vec<Spanned<MaybeExpectedEndOfInputParseError<E>>>;

pub type ParseFullyResult<O, E> = Result<O, ParseFullyError<E>>;

fn parse_fully<'input, I, E, Output, ParsableError>(
    input: &'input I,
    parse: impl FnOnce(
        &mut Parser<'input, I, MaybeExpectedEndOfInputParseError<E>>,
    ) -> Result<Output, ParsableError>,
) -> ParseFullyResult<Output, E>
where
    I: Input + ?Sized + 'input,
{
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    if parser.has_no_errors() && !parser.is_at_end() {
        parser.add_error(MaybeExpectedEndOfInputParseError::ExpectedEndOfInput);
    }

    let errors = parser.finish();

    if errors.is_empty() {
        result.map_err(|_| errors)
    } else {
        Err(errors)
    }
}

fn parse_fully_allow_trailing<'input, I, E, Output, ParsableError>(
    input: &'input I,
    parse: impl FnOnce(
        &mut Parser<'input, I, MaybeExpectedEndOfInputParseError<E>>,
    ) -> Result<Output, ParsableError>,
) -> ParseFullyResult<Output, E>
where
    I: Input + ?Sized + 'input,
{
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    let errors = parser.finish();

    if errors.is_empty() {
        result.map_err(|_| errors)
    } else {
        Err(errors)
    }
}

pub trait InputExt<'input>: Input {
    #[inline]
    fn parse_fully_with_context<E, P, Context>(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> ParseFullyResult<P::Output<'input, Self>, E>
    where
        MaybeExpectedEndOfInputParseError<E>: From<P::Error>,
        P: ParsableWithContext<'input, Self, MaybeExpectedEndOfInputParseError<E>, Context>,
    {
        parse_fully(self, |parser| parsable.expect_with_context(parser, context))
    }

    #[inline]
    fn parse_fully_with_context_allow_trailing<E, P, Context>(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> ParseFullyResult<P::Output<'input, Self>, E>
    where
        MaybeExpectedEndOfInputParseError<E>: From<P::Error>,
        P: ParsableWithContext<'input, Self, MaybeExpectedEndOfInputParseError<E>, Context>,
    {
        parse_fully_allow_trailing(self, |parser| parsable.expect_with_context(parser, context))
    }

    #[inline]
    fn try_parse_fully_with_context<E, P, Context>(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> ParseFullyResult<Option<P::Output<'input, Self>>, E>
    where
        MaybeExpectedEndOfInputParseError<E>: From<P::Error>,
        P: ParsableWithContext<'input, Self, MaybeExpectedEndOfInputParseError<E>, Context>,
    {
        parse_fully(self, |parser| {
            parsable.try_parse_with_context(parser, context)
        })
    }

    #[inline]
    fn try_parse_fully_with_context_allow_trailing<E, P, Context>(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> ParseFullyResult<Option<P::Output<'input, Self>>, E>
    where
        MaybeExpectedEndOfInputParseError<E>: From<P::Error>,
        P: ParsableWithContext<'input, Self, MaybeExpectedEndOfInputParseError<E>, Context>,
    {
        parse_fully_allow_trailing(self, |parser| {
            parsable.try_parse_with_context(parser, context)
        })
    }

    #[inline]
    fn parse_fully<E, P>(&'input self, parsable: &P) -> ParseFullyResult<P::Output<'input, Self>, E>
    where
        MaybeExpectedEndOfInputParseError<E>: From<P::Error>,
        P: Parsable<'input, Self, MaybeExpectedEndOfInputParseError<E>>,
    {
        parse_fully(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn parse_fully_allow_trailing<E, P>(
        &'input self,
        parsable: &P,
    ) -> ParseFullyResult<P::Output<'input, Self>, E>
    where
        MaybeExpectedEndOfInputParseError<E>: From<P::Error>,
        P: Parsable<'input, Self, MaybeExpectedEndOfInputParseError<E>>,
    {
        parse_fully_allow_trailing(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn try_parse_fully<E, P>(
        &'input self,
        parsable: &P,
    ) -> ParseFullyResult<Option<P::Output<'input, Self>>, E>
    where
        MaybeExpectedEndOfInputParseError<E>: From<P::Error>,
        P: Parsable<'input, Self, MaybeExpectedEndOfInputParseError<E>>,
    {
        parse_fully(self, |parser| parsable.try_parse(parser))
    }

    #[inline]
    fn try_parse_fully_allow_trailing<
        E,
        P: Parsable<'input, Self, MaybeExpectedEndOfInputParseError<E>>,
    >(
        &'input self,
        parsable: &P,
    ) -> ParseFullyResult<Option<P::Output<'input, Self>>, E> {
        parse_fully_allow_trailing(self, |parser| parsable.try_parse(parser))
    }
}

impl<'input, I> InputExt<'input> for I where I: Input + ?Sized + 'input {}
