use std::error::Error;
use std::fmt;

/// Errors that can occur during decoding of an encoded string.
///
/// This enum represents all possible error conditions that may arise when
/// attempting to decode a string produced by the [`crate::encode`] function.
///
/// Variants:
/// - [`DecodeError::SentinelNotFound`]: The expected sentinel character (`\0`) was not found at the correct position.
/// - [`DecodeError::LengthMarkerCorrupt`]: The length marker was missing, malformed, or could not be parsed.
/// - [`DecodeError::LengthOverflow`]: The decoded string exceeded the expected length specified by the marker.
/// - [`DecodeError::NonDigitInLengthMarker`]: A non-ASCII digit was encountered in the length marker.
/// - [`DecodeError::Truncated`]: The encoded string ended before the expected number of bytes for a string.
/// - [`DecodeError::Utf8`]: The decoded bytes could not be converted to a valid UTF-8 string.
#[derive(Debug)]
pub enum DecodeError {
    /// The expected sentinel character (`\0`) was not found.
    SentinelNotFound(),
    /// The length marker was missing or malformed.
    LengthMarkerCorrupt(),
    /// The decoded string exceeded the expected length.
    LengthOverflow(),
    /// A non-ASCII digit was found in the length marker.
    NonDigitInLengthMarker(),
    /// The encoded string ended before the expected number of bytes.
    Truncated(),
    /// The decoded bytes could not be converted to a valid UTF-8 string.
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
                write!(f, "UTF-8 Parsing error: {}", e)
            }
        }
    }
}

impl From<std::str::Utf8Error> for DecodeError {
    fn from(e: std::str::Utf8Error) -> Self { DecodeError::Utf8(e) }
}

impl Error for DecodeError {}
