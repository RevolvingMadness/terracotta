use crate::{
    parsable_traits::{Parsable, ParsableInstance},
    parser::Parser,
    result::regular::ParseResult,
};

pub trait StrExt<'input> {
    fn parse_standalone_with_context<Context, T: Parsable<'input, Context>>(
        &self,
        context: Context,
        allow_trailing: bool,
    ) -> ParseResult<T>;

    fn parse_standalone<T: Parsable<'input, ()>>(&self, allow_trailing: bool) -> ParseResult<T>;
}

impl<'input> StrExt<'input> for &'input str {
    fn parse_standalone_with_context<Context, T: Parsable<'input, Context>>(
        &self,
        context: Context,
        allow_trailing: bool,
    ) -> ParseResult<T> {
        let mut parser = Parser::new_with_context(self, context);

        let result = T::parse(&mut parser)?;

        if !allow_trailing && !parser.remaining().is_empty() {
            let failure = parser.add_error_with_length_one("Expected end of input".to_owned());

            return ParseResult::HardFailure(failure);
        }

        ParseResult::Success(result)
    }

    #[inline]
    fn parse_standalone<T: Parsable<'input, ()>>(&self, allow_trailing: bool) -> ParseResult<T> {
        self.parse_standalone_with_context((), allow_trailing)
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
