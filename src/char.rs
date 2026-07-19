use crate::{
    parsable_traits::{Parsable, ParsableInstance},
    parser::Parser,
    result::regular::ParseResult,
};

impl<Context> Parsable<'_, Context> for char {
    const NAME: &'static str = "character";

    type Output = Self;

    fn parse(parser: &mut Parser<'_, Context>) -> ParseResult<Self::Output> {
        let Some(character) = parser.peek() else {
            return ParseResult::SoftFailure;
        };

        parser.advance_len(character.len_utf8());

        ParseResult::Success(character)
    }
}

impl<Context> ParsableInstance<'_, Context> for char {
    type Output = Self;

    fn name(&self) -> String {
        self.to_string()
    }

    fn parse_instance(&self, parser: &mut Parser<'_, Context>) -> ParseResult<Self::Output> {
        if parser.peek().is_none_or(|character| character != *self) {
            return ParseResult::SoftFailure;
        }

        parser.advance_len(self.len_utf8());

        ParseResult::Success(*self)
    }
}
