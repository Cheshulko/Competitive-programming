// https://leetcode.com/problems/determine-if-two-strings-are-close

use std::iter::repeat;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let values = |word: String| {
            word.as_bytes().iter().fold(vec![0; 26], |mut v, c| {
                v[(*c - b'a') as usize] += 1;
                v
            })
        };

        let mut v1 = values(word1);
        let mut v2 = values(word2);

        if v1
            .iter()
            .zip(v2.iter())
            .any(|(a, b)| (a > &0 && b == &0) || (a == &0 && b > &0))
        {
            return false;
        }

        v1.sort_unstable();
        v2.sort_unstable();

        v1 == v2
    }
}
