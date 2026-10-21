use crate::{
    parse_error::{Endianness, ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianU16Parser;

impl<E> Parsable<'_, [u8], E> for BigEndianU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let bytes = parser.advance_len_exact(2)?;

        let bytes = <[u8; 2] as TryFrom<&[u8]>>::try_from(bytes).unwrap();

        Ok(u16::from_be_bytes(bytes))
    }
}

impl ParsableDefinition<'_> for BigEndianU16Parser {
    type Output = u16;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Short {
            signed: false,
            non_zero: false,
            endianness: Endianness::Big,
        })
    }
}
