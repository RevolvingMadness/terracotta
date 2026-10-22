use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianF32Parser;

impl ParsableDefinition<'_> for BigEndianF32Parser {
    type Output = f32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Float {
            endianness: Endianness::Big,
        })
    }
}

impl<E> Parsable<'_, [u8], E> for BigEndianF32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f32::from_be_bytes(bytes))
    }
}
