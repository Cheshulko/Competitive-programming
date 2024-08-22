// https://leetcode.com/problems/number-complement

struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        ((1 << (num.ilog2() + 1)) - 1) ^ num
    }
}
