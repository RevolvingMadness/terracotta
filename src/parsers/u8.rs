use crate::{
    input::Input,
    parsable_traits::Parsable,
    parser::{CalledFromParser, Parser},
    result::{ParseFailure, ParseResult},
};

impl<I: Input<Token = Self>, Context> Parsable<I, Context> for u8 {
    const NAME: &'static str = "unsigned 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<I, Context>, _: CalledFromParser) -> ParseResult<Self::Output> {
        let Some(character) = parser.peek() else {
            return Err(ParseFailure::Soft);
        };

        parser.advance_token(&character);

        Ok(character)
    }
}
