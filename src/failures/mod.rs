use crate::failures::hard::HardParseFailure;

pub mod hard;
pub mod soft;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseFailure {
    Soft,
    Hard(HardParseFailure),
}
