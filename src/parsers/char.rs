use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableInstance},
    parser::Parser,
    result::{ParseFailure, ParseResult},
};

impl<I: Input<Token = Self>> Parsable<I> for char {
    const NAME: &'static str = "character";

    type Output = Self;

    fn parse(parser: &mut Parser<I>) -> ParseResult<Self::Output> {
        let Some(character) = parser.peek() else {
            return Err(ParseFailure::Soft);
        };

        parser.advance_token(&character);

        Ok(character)
    }
}

impl<I: Input<Token = Self>> ParsableInstance<I> for char {
    type Output = Self;

    fn name(&self) -> String {
        format!("`{}`", self)
    }

    fn instance_parse(&self, parser: &mut Parser<I>) -> ParseResult<Self::Output> {
        if parser.peek().is_none_or(|character| character != *self) {
            return Err(ParseFailure::Soft);
        }

        parser.advance_token(self);

        Ok(*self)
    }
}
