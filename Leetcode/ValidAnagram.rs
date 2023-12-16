// https://leetcode.com/problems/valid-anagram

struct Solution {}

impl Solution {
    pub fn is_anagram(mut s: String, mut t: String) -> bool {
        let mut s = s.chars().map(|c| c as u8).collect::<Vec<_>>();
        let mut t = t.chars().map(|c| c as u8).collect::<Vec<_>>();
        s.sort_unstable();
        t.sort_unstable();
        s == t
    }
}
