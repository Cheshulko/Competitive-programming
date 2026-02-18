// https://leetcode.com/problems/binary-number-with-alternating-bits

struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let n = n as i64;
        let mut p = 1;
        while n >= (p << 1) {
            if (n & p) == ((n & (p << 1)) >> 1) {
                return false;
            }
            p <<= 1;
        }

        true
    }
}
