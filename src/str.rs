use crate::{
    parsable_traits::{Parsable, ParsableInstance},
    parser::Parser,
    result::regular::ParseResult,
};

pub trait StrExt<'input> {
    fn parse_standalone_with_context<Context, T: Parsable<'input, Context>>(
        &self,
        context: Context,
    ) -> ParseResult<T>;

    fn parse_standalone<T: Parsable<'input, ()>>(&self) -> ParseResult<T>;
}

impl<'input> StrExt<'input> for &'input str {
    fn parse_standalone_with_context<Context, T: Parsable<'input, Context>>(
        &self,
        context: Context,
    ) -> ParseResult<T> {
        let mut parser = Parser::new_with_context(self, context);

        T::parse(&mut parser)
    }

    #[inline]
    fn parse_standalone<T: Parsable<'input, ()>>(&self) -> ParseResult<T> {
        self.parse_standalone_with_context(())
    }
}

impl<Context> ParsableInstance<'_, '_, Context> for &str {
    fn name(&self) -> String {
        (*self).to_owned()
    }

    fn parse_instance(&self, parser: &mut Parser<'_, Context>) -> ParseResult<()> {
        if !parser.remaining().starts_with(self) {
            return ParseResult::SoftFailure;
        }

        parser.advance_len(self.len());

        ParseResult::Success(())
    }
}
