// https://leetcode.com/problems/maximum-score-after-splitting-a-string

struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        s.chars()
            .fold(
                (0, 0, s.chars().filter(|c| c == &'1').count() as i32, 1),
                |(mut mx, mut zeros, mut ones, left_len), c| {
                    zeros += (c == '0') as i32;
                    ones -= (c == '1') as i32;
                    if left_len != s.len() {
                        mx = mx.max(zeros + ones);
                    }
                    (mx, zeros, ones, left_len + 1)
                },
            )
            .0
    }
}
