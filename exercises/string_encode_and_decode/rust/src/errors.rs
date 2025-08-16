use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DecodeError {
    SentinelNotFound(),
    LengthMarkerCorrupt(),
    LengthOverflow(),
    NonDigitInLengthMarker(),
    Truncated(),
    Utf8(std::str::Utf8Error),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SentinelNotFound() => f.write_str("Sentinel character not found when expected"),
            Self::LengthMarkerCorrupt() => {
                f.write_str("Length marker was corrupt, parsing error")
            },
            Self::LengthOverflow() => {
                f.write_str("Actual UTF-8 string overflowed expected length")
            },
            Self::NonDigitInLengthMarker() => {
                f.write_str("Encountered a non-ASCII digit in length marker")
            },
            Self::Truncated() => {
                f.write_str("String payload cut off before expected length")
            },
            Self::Utf8(e) => {
                f.write_str(format!("UTF-8 Parsing error: {}", e).as_str())
            }
        }
    }
}

impl From<std::str::Utf8Error> for DecodeError {
    fn from(e: std::str::Utf8Error) -> Self { DecodeError::Utf8(e) }
}

impl Error for DecodeError {}
