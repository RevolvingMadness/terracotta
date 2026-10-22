use crate::{
    errors::number::ExpectedNumberParseError,
    input::Input,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    results::{ParseResult, option::OptionTExt},
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct U8Parser;

impl ParsableDefinition for U8Parser {
    type Output<'input, I: Input + ?Sized + 'input> = u8;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Byte {
            signed: false,
            non_zero: false,
        })
    }
}

impl<'input, E> Parsable<'input, [u8], E> for U8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let value = parser.advance().into_parse_result()?;

        Ok(value)
    }
}
