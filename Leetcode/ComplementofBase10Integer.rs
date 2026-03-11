// https://leetcode.com/problems/complement-of-base-10-integer

struct Solution;

impl Solution {
    pub fn bitwise_complement(mut n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let mut ans = 0;
        let mut p = 1;
        while p <= n {
            ans |= (n & p ^ p);
            p <<= 1;
        }

        ans
    }
}
