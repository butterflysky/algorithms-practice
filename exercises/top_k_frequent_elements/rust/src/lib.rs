use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i64>, k: usize) -> Vec<i64> {
    if k == 0 || nums.is_empty() {
        return vec![];
    }

    let mut freq_map: HashMap<i64, usize> = HashMap::with_capacity(nums.len());
    let mut max_freq: usize = 0;

    for n in nums {
        let count = freq_map.entry(n).or_insert(0);
        *count += 1;
        if *count > max_freq {
            max_freq = *count;
        }
    }

    let mut buckets: Vec<Vec<i64>> = vec![Vec::new(); max_freq + 1];

    for (val, cnt) in freq_map {
        buckets[cnt].push(val);
    }

    buckets.into_iter().rev().flatten().take(k).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums: Vec<i64> = vec![
            1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7,
        ];
        let expected: Vec<i64> = vec![5, 6, 7];
        let mut out = top_k_frequent(nums, 3);
        out.sort();
        assert_eq!(out, expected);
    }
}
