// https://leetcode.com/problems/number-of-1-bits

struct Solution {}

impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            ans += n % 2;
            n /= 2;
        }
        ans as i32
    }
}
