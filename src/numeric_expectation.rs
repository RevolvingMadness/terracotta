use core::fmt::{self, Display, Formatter};

use crate::endianness::Endianness;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericExpectation {
    Byte {
        signed: bool,
        non_zero: bool,
    },
    Short {
        signed: bool,
        non_zero: bool,
        endianness: Endianness,
    },
    Integer {
        signed: bool,
        non_zero: bool,
        endianness: Endianness,
    },
    Long {
        signed: bool,
        non_zero: bool,
        endianness: Endianness,
    },
    Float {
        endianness: Endianness,
    },
    Double {
        endianness: Endianness,
    },
}

impl Display for NumericExpectation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Byte { signed, non_zero } => {
                if non_zero {
                    write!(f, "non-zero ")?;
                }

                if signed {
                    write!(f, "signed ")?;
                } else {
                    write!(f, "unsigned ")?;
                }

                write!(f, "8-bit integer")
            }
            Self::Short {
                signed,
                non_zero,
                endianness,
            } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian ")?;

                if non_zero {
                    write!(f, "non-zero ")?;
                }

                if signed {
                    write!(f, "signed ")?;
                } else {
                    write!(f, "unsigned ")?;
                }

                write!(f, "8-bit integer")
            }
            Self::Integer {
                signed,
                non_zero,
                endianness,
            } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian ")?;

                if non_zero {
                    write!(f, "non-zero ")?;
                }

                if signed {
                    write!(f, "signed ")?;
                } else {
                    write!(f, "unsigned ")?;
                }

                write!(f, "8-bit integer")
            }
            Self::Long {
                signed,
                non_zero,
                endianness,
            } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian ")?;

                if non_zero {
                    write!(f, "non-zero ")?;
                }

                if signed {
                    write!(f, "signed ")?;
                } else {
                    write!(f, "unsigned ")?;
                }

                write!(f, "8-bit integer")
            }
            Self::Float { endianness } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian float")
            }
            Self::Double { endianness } => {
                match endianness {
                    Endianness::Little => {
                        write!(f, "little")?;
                    }
                    Endianness::Big => {
                        write!(f, "big")?;
                    }
                }

                write!(f, "-endian float")
            }
        }
    }
}
