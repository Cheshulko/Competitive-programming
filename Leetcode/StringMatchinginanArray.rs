// https://leetcode.com/problems/string-matching-in-an-array

struct Solution {}

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        words
            .iter()
            .enumerate()
            .filter_map(|(ind, target)| {
                words
                    .iter()
                    .take(ind)
                    .chain(words.iter().skip(ind + 1))
                    .any(|source| source.contains(target))
                    .then_some(target.to_owned())
            })
            .collect()
    }
}
