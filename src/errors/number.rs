use crate::numeric_expectation::NumericExpectation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpectedNumberParseError(pub NumericExpectation);
