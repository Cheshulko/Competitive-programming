// https://leetcode.com/problems/first-unique-character-in-a-string

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let unq = s
            .as_bytes()
            .iter()
            .fold(HashMap::<&u8, usize>::new(), |mut hm, c| {
                *hm.entry(c).or_default() += 1;
                hm
            });
        s.as_bytes()
            .iter()
            .position(|c| unq.get(c).unwrap() == &1)
            .map(|p| p as i32)
            .unwrap_or(-1)
    }
}
