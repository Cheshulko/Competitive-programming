// https://leetcode.com/problems/backspace-string-compare

struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_ = vec![];
        let mut t_ = vec![];

        s.chars().for_each(|c| {
            if c == '#' {
                s_.pop();
            } else {
                s_.push(c);
            }
        });
        t.chars().for_each(|c| {
            if c == '#' {
                t_.pop();
            } else {
                t_.push(c);
            }
        });

        s_ == t_
    }
}
