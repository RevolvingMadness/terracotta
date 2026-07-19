use crate::{
    parsable_traits::{Parsable, ParsableInstance},
    parser::{FullParseResult, Parser},
    result::{ParseFailure, ParseResult},
};

pub trait StrExt<'input> {
    fn parse_standalone_with_context<Context, P: Parsable<'input, Context>>(
        &self,
        context: Context,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output>;

    fn parse_standalone<P: Parsable<'input, ()>>(
        &self,
        allow_trailing: bool,
    ) -> FullParseResult<(), P::Output>;
}

impl<'input> StrExt<'input> for &'input str {
    fn parse_standalone_with_context<Context, P: Parsable<'input, Context>>(
        &self,
        context: Context,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output> {
        let parser = Parser::new_with_context(self, context);

        parser.parse_fully::<P>(allow_trailing)
    }

    #[inline]
    fn parse_standalone<P: Parsable<'input, ()>>(
        &self,
        allow_trailing: bool,
    ) -> FullParseResult<(), P::Output> {
        self.parse_standalone_with_context::<_, P>((), allow_trailing)
    }
}

impl<Context> ParsableInstance<'_, Context> for &str {
    type Output = Self;

    fn name(&self) -> String {
        format!("`{}`", self)
    }

    fn parse_instance(&self, parser: &mut Parser<'_, Context>) -> ParseResult<Self::Output> {
        if !parser.remaining().starts_with(self) {
            return Err(ParseFailure::Soft);
        }

        parser.advance_str(self);

        Ok(self)
    }
}
