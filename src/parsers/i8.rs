use crate::{
    parsable_traits::Parsable,
    parser::Parser,
    result::{OptionTExt, ParseResult},
};

impl<'input, Context> Parsable<&'input [u8], Context> for i8 {
    const NAME: &'static str = "signed 8-bit integer";

    type Output = u8;

    fn parse(parser: &mut Parser<&'input [u8], Context>) -> ParseResult<Self::Output> {
        let value = parser.advance().into_parse_result_soft()?;

        Ok(*value)
    }
}
