use crate::{
    parse_error::{Endianness, ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianF32Parser;

impl<E> Parsable<'_, [u8], E> for LittleEndianF32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(4)?;

        let bytes = <[u8; 4] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(f32::from_le_bytes(bytes))
    }
}

impl ParsableDefinition<'_> for LittleEndianF32Parser {
    type Output = f32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Float {
            endianness: Endianness::Little,
        })
    }
}
