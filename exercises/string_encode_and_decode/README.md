# Encode and Decode Strings

Design an algorithm to encode a list of strings to a single string. The encoded string is then decoded back to the original list of strings.

Please implement encode and decode

## Constraints

* 0 <= strs.length < 100
* 0 <= strs[i].length < 200
* strs[i] contains only UTF-8 characters

## Target

Aim for O(m) time with O(m+n) space for each encode() and decode() call, where:
* m is the sum of lengths of all the strings
* n is the number of strings

## Thought Process

### First attempt

* Using a delimiter character to separate strings is tricky, since any character could appear in the input. To avoid ambiguity, I plan to encode the length of each string as part of the delimiter.
    * For example, I could use something like `(\0 + str.len().to_string() + \0)` before each string.
* In Rust, strings are UTF-8 encoded, so you can't index directly into them by character. Operations like `String.nth()` are O(n) because you have to walk through the string to find the nth character.
* Allocating a fixed-size buffer for each character (like 4 bytes per char) would waste space.
* That's why I'm considering a delimiter-based approach with the string length encoded.
* For decoding, I'll need to scan the encoded string byte by byte, extract each length, and then read that many bytes for each string. This is essentially a simple state machine.

### Decoding edge case

* My first implementation failed for the input `[""]` (a single empty string): decoding returned an empty list instead of a list with one empty string. I need to handle empty strings correctly.

## Implementation Notes

* I implemented custom error types in a separate module.
* The `encode` and `decode` functions are generic: they accept any type that implements `IntoIterator`, and the items can be converted to string references (`AsRef<str>`). This allows flexibility for different input types.

