use crate::{
    parsable_traits::{Parsable, ParsableInstance},
    parser::Parser,
    result::{hard::HardParseResult, regular::ParseResult},
};

pub trait StrExt<'input> {
    fn parse_standalone_with_context<Context, P: Parsable<'input, Context>>(
        &self,
        context: Context,
        allow_trailing: bool,
    ) -> (Parser<'input, Context>, HardParseResult<Option<P::Output>>);

    fn parse_standalone<P: Parsable<'input, ()>>(
        &self,
        allow_trailing: bool,
    ) -> (Parser<'input, ()>, HardParseResult<Option<P::Output>>);
}

impl<'input> StrExt<'input> for &'input str {
    fn parse_standalone_with_context<Context, P: Parsable<'input, Context>>(
        &self,
        context: Context,
        allow_trailing: bool,
    ) -> (Parser<'input, Context>, HardParseResult<Option<P::Output>>) {
        let mut parser = Parser::new_with_context(self, context);

        let result = P::try_parse(&mut parser);

        let result = 'result: {
            if !allow_trailing && !parser.remaining().is_empty() {
                let failure = parser.add_error_with_length_one("Expected end of input".to_owned());

                break 'result HardParseResult::Failure(failure);
            }

            result
        };

        (parser, result)
    }

    #[inline]
    fn parse_standalone<P: Parsable<'input, ()>>(
        &self,
        allow_trailing: bool,
    ) -> (Parser<'input, ()>, HardParseResult<Option<P::Output>>) {
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
            return ParseResult::SoftFailure;
        }

        parser.advance_len(self.len());

        ParseResult::Success(self)
    }
}
