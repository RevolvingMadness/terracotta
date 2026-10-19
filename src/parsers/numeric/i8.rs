use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    result::ParseResult,
};

pub struct I8Parser;

impl ParsableDefinition<'_> for I8Parser {
    type Output = i8;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::I8
    }
}

impl Parsable<'_, [u8]> for I8Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = parser.advance_result()?;

        Ok(value as i8)
    }
}
