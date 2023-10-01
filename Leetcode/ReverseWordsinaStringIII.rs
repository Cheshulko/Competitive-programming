// https://leetcode.com/problems/reverse-words-in-a-string-iii

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.chars()
            .rev()
            .collect::<String>()
            .split(' ')
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}
