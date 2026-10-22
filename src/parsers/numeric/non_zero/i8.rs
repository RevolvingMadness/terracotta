use core::num::NonZeroI8;

use crate::{
    errors::number::ExpectedNumberParseError,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::i8::I8Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct NonZeroI8Parser;

impl ParsableDefinition for NonZeroI8Parser {
    type Output<'input> = NonZeroI8;

    type Error = ExpectedNumberParseError;

    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Byte {
            signed: true,
            non_zero: true,
        })
    }
}

impl<'input, E: From<ExpectedNumberParseError>> Parsable<'input, [u8], E> for NonZeroI8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input>> {
        let start = parser.position;

        let value = OptionalNonZeroI8Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Byte {
                        signed: true,
                        non_zero: true,
                    }),
                )
                .into_parse_result();
        };

        Ok(value)
    }
}

pub struct OptionalNonZeroI8Parser;

impl ParsableDefinition for OptionalNonZeroI8Parser {
    type Output<'input> = Option<NonZeroI8>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Byte {
            signed: true,
            non_zero: false,
        })
    }
}

impl<'input, E> Parsable<'input, [u8], E> for OptionalNonZeroI8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input>> {
        let value = I8Parser.parse(parser)?;

        Ok(NonZeroI8::new(value))
    }
}
