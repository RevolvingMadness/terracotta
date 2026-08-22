use crate::{
    input::Input,
    parsable_traits::Parsable,
    parser::{FullParseResult, Parser},
};

pub trait InputExt<I: Input> {
    fn parse_standalone_with_context<Context, P: Parsable<I, Context>>(
        self,
        context: Context,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output>;

    fn parse_standalone<P: Parsable<I, ()>>(
        self,
        allow_trailing: bool,
    ) -> FullParseResult<(), P::Output>;
}

impl<I: Input> InputExt<I> for I {
    fn parse_standalone_with_context<Context, P: Parsable<I, Context>>(
        self,
        context: Context,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output> {
        let parser = Parser::new_with_context(self, context);

        parser.parse_fully::<P>(allow_trailing)
    }

    #[inline]
    fn parse_standalone<P: Parsable<I, ()>>(
        self,
        allow_trailing: bool,
    ) -> FullParseResult<(), P::Output> {
        self.parse_standalone_with_context::<(), P>((), allow_trailing)
    }
}
