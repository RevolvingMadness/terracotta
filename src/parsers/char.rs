use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableInstance},
    parser::Parser,
    result::{ParseFailure, ParseResult},
};

impl<I: Input<Token = Self> + ?Sized> Parsable<'_, I> for char {
    const NAME: &'static str = "character";

    type Output = Self;

    fn parse(parser: &mut Parser<I>) -> ParseResult<Self::Output> {
        let Ok(character) = parser.peek() else {
            return Err(ParseFailure::Soft);
        };

        parser.advance_token(&character);

        Ok(character)
    }
}

impl<I: Input<Token = Self> + ?Sized> ParsableInstance<'_, I> for char {
    type Output = Self;

    fn name(&self) -> String {
        format!("`{}`", self)
    }

    fn instance_parse(&self, parser: &mut Parser<I>) -> ParseResult<Self::Output> {
        let Ok(character) = parser.peek() else {
            return Err(ParseFailure::Soft);
        };

        if character != *self {
            return Err(ParseFailure::Soft);
        }

        parser.advance_token(self);

        Ok(*self)
    }
}
