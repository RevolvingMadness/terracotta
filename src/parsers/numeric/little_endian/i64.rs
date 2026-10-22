use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    input::Input,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianI64Parser;

impl<'input, E> Parsable<'input, [u8], E> for LittleEndianI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(i64::from_le_bytes(bytes))
    }
}

impl ParsableDefinition for LittleEndianI64Parser {
    type Output<'input, I: Input + ?Sized + 'input> = i64;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Long {
            signed: true,
            non_zero: false,
            endianness: Endianness::Little,
        })
    }
}
