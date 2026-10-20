use crate::failures::ParseFailure;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoftParseFailure;

impl From<SoftParseFailure> for ParseFailure {
    fn from(SoftParseFailure: SoftParseFailure) -> Self {
        Self::Soft
    }
}
