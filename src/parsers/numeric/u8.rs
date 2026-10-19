use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    result::ParseResult,
};

pub struct U8Parser;

impl ParsableDefinition<'_> for U8Parser {
    type Output = u8;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::U8
    }
}

impl Parsable<'_, [u8]> for U8Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = parser.advance_result()?;

        Ok(value)
    }
}
