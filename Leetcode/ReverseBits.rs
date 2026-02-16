// https://leetcode.com/problems/reverse-bits

struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut ans = 0;
        for b in 0..i32::BITS {
            let b = ((n & (1 << b)) >> b) << (i32::BITS - b - 1);
            ans |= b;
        }

        ans
    }
}
