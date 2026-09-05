use crate::{
    input::Input,
    parsable_traits::{
        Parsable, ParsableInstance, ParsableInstanceWithContext, ParsableWithContext,
    },
    parser::{FullParseResult, Parser},
};

pub trait InputExt<'input>: Input {
    fn parse_fully_with_context<P: ParsableWithContext<'input, Self, Context>, Context>(
        &'input self,
        context: &mut Context,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output>;

    fn parse_fully<P: Parsable<'input, Self>>(
        &'input self,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output>;

    fn parse_instance_fully_with_context<
        P: ParsableInstanceWithContext<'input, Self, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output>;

    fn parse_instance_fully<P: ParsableInstance<'input, Self>>(
        &'input self,
        parsable: &P,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output>;
}

impl<'input, I: Input + ?Sized> InputExt<'input> for I {
    fn parse_fully_with_context<P: ParsableWithContext<'input, Self, Context>, Context>(
        &'input self,
        context: &mut Context,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output> {
        let mut parser = Parser::new(self);

        let result = P::expect_with_context(&mut parser, context);

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let errors = parser.finish();

        FullParseResult { errors, output }
    }

    #[inline]
    fn parse_fully<P: Parsable<'input, Self>>(
        &'input self,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output> {
        let mut parser = Parser::new(self);

        let result = P::expect(&mut parser);

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let errors = parser.finish();

        FullParseResult { errors, output }
    }

    fn parse_instance_fully_with_context<
        P: ParsableInstanceWithContext<'input, Self, Context>,
        Context,
    >(
        &'input self,
        parsable: &P,
        context: &mut Context,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output> {
        let mut parser = Parser::new(self);

        let result = parsable.instance_expect_with_context(&mut parser, context);

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let errors = parser.finish();

        FullParseResult { errors, output }
    }

    #[inline]
    fn parse_instance_fully<P: ParsableInstance<'input, Self>>(
        &'input self,
        parsable: &P,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output> {
        let mut parser = Parser::new(self);

        let result = parsable.instance_expect(&mut parser);

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let errors = parser.finish();

        FullParseResult { errors, output }
    }
}
