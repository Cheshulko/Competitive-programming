// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones

struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        !s.into_bytes()
            .windows(2)
            .any(|w| w[0] == b'0' && w[1] == b'1')
    }
}
