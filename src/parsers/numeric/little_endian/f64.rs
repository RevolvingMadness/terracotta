use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    input::Input,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianF64Parser;

impl ParsableDefinition for LittleEndianF64Parser {
    type Output<'input, I: Input + ?Sized + 'input> = f64;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Double {
            endianness: Endianness::Little,
        })
    }
}

impl<'input, E> Parsable<'input, [u8], E> for LittleEndianF64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let bytes = parser.advance_len_exact(8)?;

        let bytes = <[u8; 8] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f64::from_le_bytes(bytes))
    }
}
