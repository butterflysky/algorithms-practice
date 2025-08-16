mod errors;

pub use errors::DecodeError;

enum DecoderState {
    SeekingSentinel,
    ReadingLength,
    ReadingString,
}

pub fn encode<I, T>(strs: I) -> String
where
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
    T: AsRef<str> + std::fmt::Debug,
{
    // this is overkill if we have short strings, but all told, a
    // max strs length of 100 * 200 * 4 bytes is 80KiB, negligible.
    let it = strs.into_iter();
    let mut encoded_string = String::with_capacity(it.len() * 200 * 4);

    for s in it {
        encoded_string.push('\0');
        encoded_string.push_str(&(s.as_ref().len()).to_string());
        encoded_string.push('\0');
        encoded_string.push_str(s.as_ref());
    }
    encoded_string
}

pub fn decode<I>(in_str: I) -> Result<Vec<String>, DecodeError>
where
    I: AsRef<str> + std::fmt::Debug,
{
    let mut out: Vec<String> = Vec::with_capacity(100);

    let mut current_string = Vec::<u8>::with_capacity(200);
    let mut expected_len: usize = 0;

    let mut state: DecoderState = DecoderState::SeekingSentinel;

    for b in in_str.as_ref().as_bytes() {
        match state {
            DecoderState::SeekingSentinel => {
                if *b != 0 {

                    return Err(DecodeError::SentinelNotFound());
                }
                // this is the starting null, on to reading length
                state = DecoderState::ReadingLength;
            }

            DecoderState::ReadingLength => {
                if *b == 0 {
                    // finished reading length, now prepare for payload
                    if expected_len == 0 {
                        // edge case, if this is the last string in the input,
                        // we'll finish in ReadingString state instead of SeekingSentinel
                        // - so let's handle that here
                        out.push("".to_owned());
                        state = DecoderState::SeekingSentinel;
                        continue;
                    }
                    current_string.reserve(expected_len);
                    state = DecoderState::ReadingString;
                    continue;
                }

                if !b.is_ascii_digit() {
                    return Err(DecodeError::NonDigitInLengthMarker());
                }

                let d = (*b - b'0') as usize;
                expected_len = expected_len
                    .checked_mul(10)
                    .and_then(|v| v.checked_add(d))
                    .ok_or(DecodeError::LengthMarkerCorrupt())?;
            }

            DecoderState::ReadingString => {
                current_string.push(*b);

                if current_string.len() > expected_len {
                    return Err(DecodeError::LengthOverflow());
                }

                if current_string.len() < expected_len {
                    continue;
                }

                let s = std::str::from_utf8(&current_string)?;
                out.push(s.to_owned());

                // reset for next run
                current_string.clear();
                expected_len = 0;
                state = DecoderState::SeekingSentinel;
            }
        }
    }

    match state {
        DecoderState::SeekingSentinel => Ok(out),
        DecoderState::ReadingLength => Err(DecodeError::LengthMarkerCorrupt()),
        DecoderState::ReadingString => Err(DecodeError::Truncated()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::collection::vec;
    use proptest::prelude::*;

    fn get_simple_test_tuple() -> (Vec<String>, String, String) {
        let input = vec!["hi".to_owned(), "there".to_owned()];
        (input.clone(), "\x002\x00hi\x005\x00there".to_owned(), encode(input))
    }

    #[test]
    fn validate_encoding() {
        let (_, expected_encoding, actual_encoding) = get_simple_test_tuple();

        assert_eq!(expected_encoding, actual_encoding);
    }

    #[test]
    fn validate_decoding() {
        let (input, expected_encoding, _) = get_simple_test_tuple();

        assert_eq!(input, decode(expected_encoding).unwrap());
    }

    #[test]
    fn validate_decoding_encoding() {
        let (input, _, actual_encoding) = get_simple_test_tuple();

        assert_eq!(input, decode(actual_encoding).unwrap());
    }

    #[test]
    fn validate_encode_with_single_empty_string() {
        let result = encode(vec!["".to_owned()]);
        let mut expected: String = String::with_capacity(3);

        expected.push_str("\x000\x00");

        assert_eq!(expected.len(), 3);
        assert_eq!(result, expected);
    }

    #[test]
    fn validate_decode_with_single_empty_string() {
        let input = vec!["".to_owned()];
        let result = decode(encode(input.clone())).unwrap();

        assert_eq!(input, result);
    }

    fn arb_input_str_collection() -> impl Strategy<Value = Vec<String>> {
        let string_0_200 =
            vec(any::<char>(), 0..=200).prop_map(|chars| chars.into_iter().collect::<String>());
        vec(string_0_200, 0..=100)
    }

    proptest! {
        // with_cases(256) is default, but including this so I build muscle-memory for how to do it
        #![proptest_config(ProptestConfig::with_cases(256))]

        // Properties to test:
        // * input always equals output
        #[test]
        fn prop_decode_encode_gives_back_original(input in arb_input_str_collection()) {
            let result = decode(encode(input.clone()));

            prop_assert_eq!(input, result?);
        }
    }
}
