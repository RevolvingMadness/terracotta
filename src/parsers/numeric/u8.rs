use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct U8Parser;

impl ParsableDefinition<'_> for U8Parser {
    type Output = u8;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::U8)
    }
}

impl<E> Parsable<'_, [u8], E> for U8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = parser.advance_result()?;

        Ok(value)
    }
}
