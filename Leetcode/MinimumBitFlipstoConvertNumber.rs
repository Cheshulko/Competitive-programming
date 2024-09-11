// https://leetcode.com/problems/minimum-bit-flips-to-convert-number

struct Solution {}

impl Solution {
    pub fn min_bit_flips(mut start: i32, mut goal: i32) -> i32 {
        let mut cnt = 0;

        while start > 0 || goal > 0 {
            cnt += (start & 1) ^ (goal & 1);
            start >>= 1;
            goal >>= 1;
        }

        cnt
    }
}
