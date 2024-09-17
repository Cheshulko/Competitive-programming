// https://leetcode.com/problems/uncommon-words-from-two-sentences

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let s1 = s1
            .split_whitespace()
            .fold(HashMap::<&str, usize>::new(), |mut hm, x| {
                *hm.entry(x).or_default() += 1;
                hm
            });
        let s2 = s2
            .split_whitespace()
            .fold(HashMap::<&str, usize>::new(), |mut hm, x| {
                *hm.entry(x).or_default() += 1;
                hm
            });

        let mut ans = vec![];
        for (s, &c) in s1.iter() {
            if c > 1 {
                continue;
            }
            if s2.get(s).is_none() {
                ans.push(s.to_string());
            }
        }
        for (s, &c) in s2.iter() {
            if c > 1 {
                continue;
            }
            if s1.get(s).is_none() {
                ans.push(s.to_string());
            }
        }

        ans
    }
}
