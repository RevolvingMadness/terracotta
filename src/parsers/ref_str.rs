use crate::{
    parsable_traits::ParsableInstance,
    parser::Parser,
    result::{ParseFailure, ParseResult},
};

impl<'input> ParsableInstance<&'input str> for &'input str {
    type Output = Self;

    fn name(&self) -> String {
        format!("`{}`", self)
    }

    fn instance_parse(&self, parser: &mut Parser<&'input str>) -> ParseResult<Self::Output> {
        if !parser.slice_starts_with(parser.position(), self) {
            return Err(ParseFailure::Soft);
        }

        parser.advance_token(self);

        Ok(self)
    }
}
