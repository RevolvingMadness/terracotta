use crate::{
    input::Input,
    parsable_traits::Parsable,
    parser::{CalledFromParser, Parser},
    result::{ParseFailure, ParseResult},
};

impl<I: Input<Token = u8>, Context> Parsable<I, Context> for i8 {
    const NAME: &'static str = "signed 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<I, Context>, _: CalledFromParser) -> ParseResult<Self::Output> {
        let Some(character) = parser.peek() else {
            return Err(ParseFailure::Soft);
        };

        parser.advance_token(&character);

        Ok(character as Self)
    }
}
