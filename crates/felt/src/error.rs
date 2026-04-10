use std::error::Error;

/// Error returned when a byte slice exceeds 32 bytes, or when
/// [`Felt::valid`](crate::Felt::valid) finds the value is not in `[0, p)`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OverflowError;

impl Error for OverflowError {}

// TryFrom<Felt> for primitive
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PrimitiveFromFeltError;

impl Error for PrimitiveFromFeltError {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StrError {
    InvalidNibble(u8),
    InvalidDigit(u8),
    InvalidLength { max: usize, actual: usize },
    Overflow(u64),
    EmptyString,
    NonAsciiCharacter,
}

impl Error for StrError {}

impl std::fmt::Display for StrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNibble(n) => write!(f, "invalid nibble: 0x{:x}", *n),
            Self::InvalidDigit(d) => write!(f, "invalid digit: '{}'", *d as char),
            Self::InvalidLength { max, actual } => {
                write!(f, "more than {} digits found: {}", *max, *actual)
            }
            Self::Overflow(o) => write!(f, "The maximum field value was exceeded by: {}", *o),
            Self::EmptyString => f.write_str("empty string"),
            Self::NonAsciiCharacter => f.write_str("non-ASCII character in short string"),
        }
    }
}

impl core::fmt::Display for PrimitiveFromFeltError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed to convert `Felt` into primitive type")
    }
}

const OVERFLOW_MSG: &str = "The maximum field value was exceeded.";

impl std::fmt::Display for OverflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(OVERFLOW_MSG)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeltError {
    StrError(StrError),
    PrimitiveFromFeltError(PrimitiveFromFeltError),
    OverflowError(OverflowError),
}

impl std::fmt::Display for FeltError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StrError(e) => e.fmt(f),
            Self::PrimitiveFromFeltError(e) => e.fmt(f),
            Self::OverflowError(e) => e.fmt(f),
        }
    }
}

impl Error for FeltError {}

impl From<StrError> for FeltError {
    fn from(value: StrError) -> Self {
        Self::StrError(value)
    }
}

impl From<PrimitiveFromFeltError> for FeltError {
    fn from(value: PrimitiveFromFeltError) -> Self {
        Self::PrimitiveFromFeltError(value)
    }
}

impl From<OverflowError> for FeltError {
    fn from(value: OverflowError) -> Self {
        Self::OverflowError(value)
    }
}
