use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    results::{ParseResult, option::OptionTExt},
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct I8Parser;

impl ParsableDefinition<'_> for I8Parser {
    type Output = i8;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::I8)
    }
}

impl<E> Parsable<'_, [u8], E> for I8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = parser.advance().into_parse_result()?;

        Ok(value as i8)
    }
}
