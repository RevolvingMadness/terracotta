use crate::{
    input::Input,
    parsable_traits::{
        Parsable, ParsableInstance, ParsableInstanceWithContext, ParsableWithContext,
    },
    parser::{FullParseResult, Parser},
};

pub trait InputExt: Input + Sized {
    fn parse_fully_with_context<P: ParsableWithContext<Self, Context>, Context>(
        self,
        context: &Context,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output>;

    fn parse_fully<P: Parsable<Self>>(self, allow_trailing: bool) -> FullParseResult<P::Output>;

    fn parse_instance_fully_with_context<P: ParsableInstanceWithContext<Self, Context>, Context>(
        self,
        parsable: &P,
        context: &Context,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output>;

    fn parse_instance_fully<P: ParsableInstance<Self>>(
        self,
        parsable: &P,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output>;
}

impl<I: Input> InputExt for I {
    fn parse_fully_with_context<P: ParsableWithContext<Self, Context>, Context>(
        self,
        context: &Context,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output> {
        let mut parser = Parser::new(self);

        let result = parser.expect_with_context::<P, _>(context);

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let errors = parser.finish();

        FullParseResult { errors, output }
    }

    #[inline]
    fn parse_fully<P: Parsable<Self>>(self, allow_trailing: bool) -> FullParseResult<P::Output> {
        let mut parser = Parser::new(self);

        let result = parser.expect::<P>();

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let errors = parser.finish();

        FullParseResult { errors, output }
    }

    fn parse_instance_fully_with_context<P: ParsableInstanceWithContext<Self, Context>, Context>(
        self,
        parsable: &P,
        context: &Context,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output> {
        let mut parser = Parser::new(self);

        let result = parser.instance_expect_with_context(parsable, context);

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let errors = parser.finish();

        FullParseResult { errors, output }
    }

    #[inline]
    fn parse_instance_fully<P: ParsableInstance<Self>>(
        self,
        parsable: &P,
        allow_trailing: bool,
    ) -> FullParseResult<P::Output> {
        let mut parser = Parser::new(self);

        let result = parser.instance_expect(parsable);

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let errors = parser.finish();

        FullParseResult { errors, output }
    }
}
