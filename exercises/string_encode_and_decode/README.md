# Encode and Decode Strings

Design an algorithm to encode a list of strings to a single string. The encoded string is then decoded back to the original list of strings.

Please implement encode and decode

Constraints:

0 <= strs.length < 100
0 <= strs[i].length < 200
strs[i] contains only UTF-8 characters

Aim for O(m) time with O(m+n) space for each encode() and decode() call, where m is the sum of lengths of all the strings and n is the number of strings

## My thoughts

* Any character I might use a delimeter might be in the strings. I could use multiple characters. I should probably include the length of the string in the delimiter. Maybe (\0 + str.len().to_string() + \0) as a delimiter?
* Rust doesn't let you arbitrarily index into strings, as internally they are variable-width UTF-8 bytes over a Vec<u8> buffer - you wouldn't be able to accurately parse whatever grapheme you land on if you land in the middle. And String.nth() is O(n) each time because of this too - if you want the nth character, you have to parse each character up to it.
* Allocating a fixed 4-byte buffer for each char is a waste of space
* So that's why I want to try a delimiter-based approach with length encoded inside.
* Decoding is going to need to iterate over each character once, so it will have to iteratively build up each string and keep track of the expected length. It seems like a state machine would come in handy here.

## Testing

I'll implement a property test to make sure that for any valid input ```strs: Vector<String>```,

```rust
decode(encode(strs)) == strs
```

And my first go is yielding a failure for [""] - it's not giving back an empty string, it's giving back nothing at all.
## Other

I decided to implement my own error types and put them in a separate module. I also decided to experiment with making the functions able to accept vectors or arrays, essentially anything that implements IntoIterator, of types that can be made into string references, or rather, types that implement AsRef<str> - I'm not initially going to implement tests for different types, just playing with it for learning. But I may come back and revise later when I'm not prepping for interviews.

