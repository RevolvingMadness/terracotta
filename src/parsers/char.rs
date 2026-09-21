use alloc::{borrow::ToOwned, format, string::String};

use crate::{
    input::Input,
    parsable_traits::{Parsable, ParsableError, ParsableInstance, ParsableInstanceError},
    parse_error::DefaultParseError,
    parser::Parser,
    result::{ParseFailure, ParseResult},
};

pub trait ExpectedAnyCharacterParseError {
    #[must_use]
    fn expected_any_character() -> Self;
}

impl ExpectedAnyCharacterParseError for String {
    #[inline]
    fn expected_any_character() -> Self {
        "expected any character".to_owned()
    }
}

impl ExpectedAnyCharacterParseError for () {
    #[inline]
    fn expected_any_character() -> Self {}
}

impl ExpectedAnyCharacterParseError for DefaultParseError {
    fn expected_any_character() -> Self {
        Self::ExpectedAnyCharacter
    }
}

impl<I: Input<Token = Self> + ?Sized, E> Parsable<'_, I, E> for char {
    type Output = Self;

    fn parse(parser: &mut Parser<I, E>) -> ParseResult<Self::Output> {
        let Ok(character) = parser.peek() else {
            return Err(ParseFailure::Soft);
        };

        parser.advance_token(&character);

        Ok(character)
    }
}

impl<E: ExpectedAnyCharacterParseError> ParsableError<E> for char {
    #[inline]
    fn error() -> E {
        E::expected_any_character()
    }
}

pub trait ExpectedCharacterParseError {
    #[must_use]
    fn expected_character(character: char) -> Self;
}

impl ExpectedCharacterParseError for String {
    fn expected_character(character: char) -> Self {
        format!("expected `{}`", character)
    }
}

impl ExpectedCharacterParseError for () {
    fn expected_character(_: char) -> Self {}
}

impl ExpectedCharacterParseError for DefaultParseError {
    fn expected_character(character: char) -> Self {
        Self::ExpectedCharacter(character)
    }
}

impl<I: Input<Token = Self> + ?Sized, E> ParsableInstance<'_, I, E> for char {
    type Output = Self;

    fn instance_parse(&self, parser: &mut Parser<I, E>) -> ParseResult<Self::Output> {
        let Ok(character) = parser.peek() else {
            return Err(ParseFailure::Soft);
        };

        if character != *self {
            return Err(ParseFailure::Soft);
        }

        parser.advance_token(self);

        Ok(*self)
    }
}

impl<E: ExpectedCharacterParseError> ParsableInstanceError<E> for char {
    #[inline]
    fn error(&self) -> E {
        E::expected_character(*self)
    }
}
