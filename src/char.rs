use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableInstance},
    parser::{CalledFromParser, Parser},
    result::{ParseFailure, ParseResult},
};

impl<I: Input<Token = Self>, Context> Parsable<I, Context> for char {
    const NAME: &'static str = "character";

    type Output = Self;

    fn parse(parser: &mut Parser<I, Context>, _: CalledFromParser) -> ParseResult<Self::Output> {
        let Some(character) = parser.peek() else {
            return Err(ParseFailure::Soft);
        };

        parser.advance_token(&character);

        Ok(character)
    }
}

impl<I: Input<Token = Self>, Context> ParsableInstance<I, Context> for char {
    type Output = Self;

    fn name(&self) -> String {
        format!("`{}`", self)
    }

    fn parse_instance(&self, parser: &mut Parser<I, Context>) -> ParseResult<Self::Output> {
        if parser.peek().is_none_or(|character| character != *self) {
            return Err(ParseFailure::Soft);
        }

        parser.advance_token(self);

        Ok(*self)
    }
}
