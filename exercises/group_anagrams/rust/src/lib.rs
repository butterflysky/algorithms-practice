use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum InputError {
    StringTooLong(),
    CharOutOfRange(),
}

impl Error for InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::StringTooLong() => write!(f, "input string too long (> 100 chars)"),
            Self::CharOutOfRange() => write!(f, "char out of bounds (not [a-z])"),
        }
    }
}

fn gen_freq_map(s: &str) -> Result<[u8; 26], InputError> {
    let mut map = [0u8; 26];

    if s.len() > 100 {
        return Err(InputError::StringTooLong());
    }

    for &b in s.as_bytes() {
        if !(b'a'..=b'z').contains(&b) {
            return Err(InputError::CharOutOfRange());
        }

        map[(b - b'a') as usize] = map[(b - b'a') as usize] + 1;
    }
    Ok(map)
}

fn group_anagrams(strs: Vec<String>) -> Result<Vec<Vec<String>>, InputError> {
    let mut anagram_groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for string in strs.into_iter() {
        let freq_map = gen_freq_map(&string)?;

        anagram_groups.entry(freq_map).or_default().push(string);
    }

    Ok(anagram_groups.into_values().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::{HashMap, HashSet};

    // Canonicalize output to compare independently of ordering
    fn canonicalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for g in &mut groups {
            g.sort();
        }

        groups.sort_by_key(|g| g.first().map(|s| gen_freq_map(s).unwrap()));
        groups
    }

    // normal unit test, derived from example 1 in the exercise
    #[test]
    fn example1() {
        let input = vec![
            "act".to_string(),
            "pots".to_string(),
            "tops".to_string(),
            "cat".to_string(),
            "stop".to_string(),
            "hat".to_string(),
        ];
        let output = canonicalize(group_anagrams(input).unwrap());

        let expected: Vec<Vec<String>> = canonicalize(vec![
            vec!["hat".to_string()],
            vec!["act".to_string(), "cat".to_string()],
            vec!["pots".to_string(), "stop".to_string(), "tops".to_string()],
        ]);

        assert_eq!(output, expected);
    }

    // property tests and helpers

    // properties i want to test:
    //   - partition - output is a permutation (by multiset) of input
    //   - groups - all strings in each group share the same frequency map, and each group's frequency map is unique
    //   - concatenation - group(A ++ B) == merge(group(A), group(B)) by frequency map

    fn lower_ascii_vec() -> impl Strategy<Value = Vec<String>> {
        prop::collection::vec(
            proptest::string::string_regex("[a-z]{0,100}").unwrap(),
            0..1000,
        )
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(500))]

        // 1) Partition: output is a permutation (by multiset) of input
        #[test]
        fn prop_partition(strs in lower_ascii_vec()) {
            let out = group_anagrams(strs.clone()).unwrap();
            let mut flat = out.into_iter().flatten().collect::<Vec<_>>();

            let mut input_sorted = strs.clone();
            input_sorted.sort();
            flat.sort();

            prop_assert_eq!(flat, input_sorted);
        }

        // 2) Groups: all strings in each group share the same frequency map, and each group's frequency map is unique
        #[test]
        fn prop_groups_have_distinct_freqmaps_and_all_members_share_it(strs in lower_ascii_vec()) {
            let out = group_anagrams(strs.clone()).unwrap();
            let mut seen_freq_maps = HashSet::new();

            for group in out.iter() {
                let mut seen_freq_maps_in_group = HashSet::new();
                for s in group.iter() {
                    let map = gen_freq_map(&s).unwrap();
                    seen_freq_maps_in_group.insert(map);
                    seen_freq_maps.insert(map);
                }
                prop_assert_eq!(seen_freq_maps_in_group.len(), 1);
            }

            prop_assert_eq!(seen_freq_maps.len(), out.len());
        }

        // 3) Concatenation equivalence - could we safely shard the input (map), group locally, then uion the groups (reduce) without changing semantics
        #[test]
        fn prop_concat_equivalence(a in lower_ascii_vec(), b in lower_ascii_vec()) {
            // directly group concatenated input
            let out_ab = canonicalize(group_anagrams({
                let mut c = a.clone();
                c.extend(b.clone());
                c
            }).unwrap());

            // compose the merged result from individual freq maps
            let mut spec: HashMap<[u8;26], Vec<String>> = HashMap::new();
            for s in a.iter().chain(b.iter()) {
                spec.entry(gen_freq_map(&s).unwrap()).or_default().push(s.clone());
            }
            let spec_groups = canonicalize(spec.into_values().collect::<Vec<_>>());

            prop_assert_eq!(out_ab, spec_groups);
        }
    }
}
