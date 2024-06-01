// https://leetcode.com/problems/score-of-a-string

struct Solution {}

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.into_bytes()
            .windows(2)
            .fold(0, |x, v| x + v[0].abs_diff(v[1]) as i32)
    }
}
