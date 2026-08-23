use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableInstance},
    parser::{FullParseResult, Parser},
};

pub trait InputExt: Input + Sized {
    fn parse_fully_with_context<Context, P: Parsable<Self, Context>>(
        self,
        context: Context,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output>;

    fn parse_fully<P: Parsable<Self, ()>>(
        self,
        allow_trailing: bool,
    ) -> FullParseResult<(), P::Output>;

    fn parse_instance_fully_with_context<Context, P: ParsableInstance<Self, Context>>(
        self,
        context: Context,
        parsable: &P,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output>;

    fn parse_instance_fully<P: ParsableInstance<Self, ()>>(
        self,
        parsable: &P,
        allow_trailing: bool,
    ) -> FullParseResult<(), P::Output>;
}

impl<I: Input> InputExt for I {
    fn parse_fully_with_context<Context, P: Parsable<Self, Context>>(
        self,
        context: Context,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output> {
        let mut parser = Parser::new_with_context(self, context);

        let result = parser.expect::<P>();

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let (context, errors) = parser.finish();

        FullParseResult {
            context,
            errors,
            output,
        }
    }

    #[inline]
    fn parse_fully<P: Parsable<Self, ()>>(
        self,
        allow_trailing: bool,
    ) -> FullParseResult<(), P::Output> {
        self.parse_fully_with_context::<(), P>((), allow_trailing)
    }

    fn parse_instance_fully_with_context<Context, P: ParsableInstance<Self, Context>>(
        self,
        context: Context,
        parsable: &P,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output> {
        let mut parser = Parser::new_with_context(self, context);

        let result = parser.expect_instance(parsable);

        if !allow_trailing && parser.has_no_errors() && parser.position() < parser.end_position() {
            parser.add_error("Expected end of input".to_owned());
        }

        let output = result.ok();

        let (context, errors) = parser.finish();

        FullParseResult {
            context,
            errors,
            output,
        }
    }

    #[inline]
    fn parse_instance_fully<P: ParsableInstance<Self, ()>>(
        self,
        parsable: &P,
        allow_trailing: bool,
    ) -> FullParseResult<(), P::Output> {
        self.parse_instance_fully_with_context::<(), _>((), parsable, allow_trailing)
    }
}
