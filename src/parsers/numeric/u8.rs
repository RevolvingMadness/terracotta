use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

impl<'input> Parsable<&'input [Self]> for u8 {
    const NAME: &'static str = "unsigned 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<&'input [Self]>) -> ParseResult<Self::Output> {
        let value = parser.advance().into_parse_result_soft()?;

        Ok(*value)
    }
}
