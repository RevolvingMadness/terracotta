use core::num::NonZeroU8;

use crate::{
    errors::number::ExpectedNumberParseError,
    input::Input,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::u8::U8Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct NonZeroU8Parser;

impl<'input, E: From<ExpectedNumberParseError>> Parsable<'input, [u8], E> for NonZeroU8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let start = parser.position;

        let value = OptionalNonZeroU8Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Byte {
                        signed: false,
                        non_zero: true,
                    }),
                )
                .into_parse_result();
        };

        Ok(value)
    }
}

impl ParsableDefinition for NonZeroU8Parser {
    type Output<'input, I: Input + ?Sized + 'input> = NonZeroU8;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Byte {
            signed: false,
            non_zero: true,
        })
    }
}

pub struct OptionalNonZeroU8Parser;

impl<'input, E> Parsable<'input, [u8], E> for OptionalNonZeroU8Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let value = U8Parser.parse(parser)?;

        Ok(NonZeroU8::new(value))
    }
}

impl ParsableDefinition for OptionalNonZeroU8Parser {
    type Output<'input, I: Input + ?Sized + 'input> = Option<NonZeroU8>;

    type Error = ExpectedNumberParseError;

    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Byte {
            signed: false,
            non_zero: false,
        })
    }
}
