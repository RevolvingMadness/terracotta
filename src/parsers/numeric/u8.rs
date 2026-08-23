use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

impl<'input, Context> Parsable<&'input [Self], Context> for u8 {
    const NAME: &'static str = "unsigned 8-bit integer";

    type Output = Self;

    fn parse(parser: &mut Parser<&'input [Self], Context>) -> ParseResult<Self::Output> {
        let value = parser.advance().into_parse_result_soft()?;

        Ok(*value)
    }
}
