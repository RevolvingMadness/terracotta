use crate::{
    input::Input,
    parsable_traits::{
        Parsable, ParsableInstance, ParsableInstanceWithContext, ParsableWithContext,
    },
    parse_error::ExpectedEndOfInputParseError,
    parser::Parser,
    result::HardParseResult,
    span::Span,
};

fn parse_fully<'input, I: Input + ?Sized, T, E: ExpectedEndOfInputParseError>(
    input: &'input I,
    parse: impl FnOnce(&mut Parser<'input, I, E>) -> HardParseResult<T>,
) -> Result<T, Vec<(Span, E)>> {
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    if parser.has_no_errors() && !parser.is_at_end() {
        parser.add_error(E::expected_end_of_input());
    }

    let errors = parser.finish();

    result.map_err(|_| errors)
}

fn parse_fully_allow_trailing<'input, I: Input + ?Sized, T, E>(
    input: &'input I,
    parse: impl FnOnce(&mut Parser<'input, I, E>) -> HardParseResult<T>,
) -> Result<T, Vec<(Span, E)>> {
    let mut parser = Parser::new(input);

    let result = parse(&mut parser);

    let errors = parser.finish();

    result.map_err(|_| errors)
}

pub trait InputExt<'input>: Input {
    #[inline]
    fn parse_fully_with_context<
        E: ExpectedEndOfInputParseError,
        P: ParsableWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        context: &mut Context,
    ) -> Result<P::Output, Vec<(Span, E)>> {
        parse_fully(self, |parser| P::expect_with_context(parser, context))
    }

    #[inline]
    fn parse_fully_with_context_allow_trailing<
        E,
        P: ParsableWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        context: &mut Context,
    ) -> Result<P::Output, Vec<(Span, E)>> {
        parse_fully_allow_trailing(self, |parser| P::expect_with_context(parser, context))
    }

    #[inline]
    fn parse_fully<E: ExpectedEndOfInputParseError, P: Parsable<'input, Self, E>>(
        &'input self,
    ) -> Result<P::Output, Vec<(Span, E)>> {
        parse_fully(self, |parser| P::expect(parser))
    }

    #[inline]
    fn parse_fully_allow_trailing<E, P: Parsable<'input, Self, E>>(
        &'input self,
    ) -> Result<P::Output, Vec<(Span, E)>> {
        parse_fully_allow_trailing(self, |parser| P::expect(parser))
    }

    #[inline]
    fn parse_instance_fully_with_context<
        E: ExpectedEndOfInputParseError,
        P: ParsableInstanceWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<P::Output, Vec<(Span, E)>> {
        parse_fully(self, |parser| {
            parsable.instance_expect_with_context(parser, context)
        })
    }

    #[inline]
    fn parse_instance_fully_with_context_allow_trailing<
        E,
        P: ParsableInstanceWithContext<'input, Self, E, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
    ) -> Result<P::Output, Vec<(Span, E)>> {
        parse_fully_allow_trailing(self, |parser| {
            parsable.instance_expect_with_context(parser, context)
        })
    }

    #[inline]
    fn parse_instance_fully<
        E: ExpectedEndOfInputParseError,
        P: ParsableInstance<'input, Self, E>,
    >(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output, Vec<(Span, E)>> {
        parse_fully(self, |parser| parsable.instance_expect(parser))
    }

    #[inline]
    fn parse_instance_fully_allow_trailing<E, P: ParsableInstance<'input, Self, E>>(
        &'input self,
        parsable: &P,
    ) -> Result<P::Output, Vec<(Span, E)>> {
        parse_fully_allow_trailing(self, |parser| parsable.instance_expect(parser))
    }
}
