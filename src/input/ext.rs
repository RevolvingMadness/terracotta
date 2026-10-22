use alloc::vec::Vec;

use crate::{
    errors::end_of_input::ExpectedEndOfInputParseError,
    input::Input,
    parser::Parser,
    spanned::Spanned,
    traits::{
        parsable::Parsable, parsable_definition::ParsableDefinition,
        parsable_with_context::ParsableWithContext,
    },
};

fn parse_fully<'input, I, E, Output, ParsableError>(
    input: &'input I,
    parse: impl FnOnce(&mut Parser<'input, I, E>) -> Result<Output, ParsableError>,
) -> Result<Output, Vec<Spanned<E>>>
where
    I: Input + ?Sized + 'input,
    E: From<ExpectedEndOfInputParseError>,
{
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    if parser.has_no_errors() && !parser.is_at_end() {
        parser.add_error(E::from(ExpectedEndOfInputParseError));
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
    parse: impl FnOnce(&mut Parser<'input, I, E>) -> Result<Output, ParsableError>,
) -> Result<Output, Vec<Spanned<E>>>
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
    fn parse_fully_with_context<
        E: From<ExpectedEndOfInputParseError>,
        P: ParsableDefinition<Error = E> + ParsableWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<P::Output<'input, Self>, Vec<Spanned<E>>> {
        parse_fully(self, |parser| parsable.expect_with_context(parser, context))
    }

    #[inline]
    fn parse_fully_with_context_allow_trailing<
        E,
        P: ParsableDefinition<Error = E> + ParsableWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<P::Output<'input, Self>, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| parsable.expect_with_context(parser, context))
    }

    #[inline]
    fn try_parse_fully_with_context<
        E: From<ExpectedEndOfInputParseError>,
        P: ParsableDefinition<Error = E> + ParsableWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<Option<P::Output<'input, Self>>, Vec<Spanned<E>>> {
        parse_fully(self, |parser| {
            parsable.try_parse_with_context(parser, context)
        })
    }

    #[inline]
    fn try_parse_fully_with_context_allow_trailing<
        E,
        P: ParsableDefinition<Error = E> + ParsableWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<Option<P::Output<'input, Self>>, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| {
            parsable.try_parse_with_context(parser, context)
        })
    }

    #[inline]
    fn parse_fully<
        E: From<ExpectedEndOfInputParseError>,
        P: ParsableDefinition<Error = E> + Parsable<'input, Self, E>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output<'input, Self>, Vec<Spanned<E>>> {
        parse_fully(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn parse_fully_allow_trailing<
        E,
        P: ParsableDefinition<Error = E> + Parsable<'input, Self, E>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output<'input, Self>, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| parsable.expect(parser))
    }

    #[inline]
    fn try_parse_fully<
        E: From<ExpectedEndOfInputParseError>,
        P: ParsableDefinition<Error = E> + Parsable<'input, Self, E>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<Option<P::Output<'input, Self>>, Vec<Spanned<E>>> {
        parse_fully(self, |parser| parsable.try_parse(parser))
    }

    #[inline]
    fn try_parse_fully_allow_trailing<
        E,
        P: ParsableDefinition<Error = E> + Parsable<'input, Self, E>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<Option<P::Output<'input, Self>>, Vec<Spanned<E>>> {
        parse_fully_allow_trailing(self, |parser| parsable.try_parse(parser))
    }
}

impl<'input, I> InputExt<'input> for I where I: Input + ?Sized + 'input {}
