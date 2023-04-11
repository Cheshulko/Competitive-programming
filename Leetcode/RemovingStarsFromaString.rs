// https://leetcode.com/problems/removing-stars-from-a-string

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut st: VecDeque<char> = VecDeque::new();
        for c in s.chars() {
            match c {
                '*' => {
                    st.pop_back();
                }
                _ => st.push_back(c),
            }
        }
        String::from_iter(st)
    }
}
