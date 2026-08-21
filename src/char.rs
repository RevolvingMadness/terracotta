use crate::{
    parsable_traits::{Parsable, ParsableInstance},
    parser::{CalledFromParser, Parser},
    result::{ParseFailure, ParseResult},
};

impl<'input, Context> Parsable<'input, Context> for char {
    const NAME: &'static str = "character";

    type Output = Self;

    fn parse(
        parser: &mut Parser<'input, Context>,
        _: CalledFromParser,
    ) -> ParseResult<Self::Output> {
        let Some(character) = parser.peek() else {
            return Err(ParseFailure::Soft);
        };

        parser.advance_char(character);

        Ok(character)
    }
}

impl<'input, Context> ParsableInstance<'input, Context> for char {
    type Output = Self;

    fn name(&self) -> String {
        format!("`{}`", self)
    }

    fn parse_instance(&self, parser: &mut Parser<'input, Context>) -> ParseResult<Self::Output> {
        if parser.peek().is_none_or(|character| character != *self) {
            return Err(ParseFailure::Soft);
        }

        parser.advance_char(*self);

        Ok(*self)
    }
}
