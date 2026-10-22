use crate::{
    errors::number::ExpectedNumberParseError,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    results::{ParseResult, option::OptionTExt},
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct I8Parser;

impl ParsableDefinition for I8Parser {
    type Output<'input> = i8;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Byte {
            signed: true,
            non_zero: false,
        })
    }
}

impl<'input, E> Parsable<'input, [u8], E> for I8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input>> {
        let value = parser.advance().into_parse_result()?;

        Ok(value as i8)
    }
}
