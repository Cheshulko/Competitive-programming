// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let n = words.len();
        words
            .join("")
            .as_bytes()
            .into_iter()
            .fold(HashMap::new(), |mut hm, c| {
                *hm.entry(c).or_insert(0) += 1;
                hm
            })
            .values()
            .all(|cnt| cnt % n == 0)
    }
}
