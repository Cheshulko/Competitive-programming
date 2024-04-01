// https://leetcode.com/problems/length-of-last-word

struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split_whitespace().last().unwrap_or("").len() as i32
    }
}
