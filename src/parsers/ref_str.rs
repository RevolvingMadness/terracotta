use crate::{
    input::Input,
    parsable_traits::ParsableInstance,
    parser::Parser,
    result::{ParseFailure, ParseResult},
};

impl<'input, Context> ParsableInstance<&'input str, Context> for &'input str {
    type Output = Self;

    fn name(&self) -> String {
        format!("`{}`", self)
    }

    fn parse_instance(
        &self,
        parser: &mut Parser<&'input str, Context>,
    ) -> ParseResult<Self::Output> {
        if !parser.input.starts_with_slice(parser.position(), self) {
            return Err(ParseFailure::Soft);
        }

        parser.advance_token(self);

        Ok(self)
    }
}
