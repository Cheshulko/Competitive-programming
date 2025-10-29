// https://leetcode.com/problems/smallest-number-with-all-set-bits

struct Solution {}

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut ans = 0;
        for i in 0.. {
            ans |= 1 << i;
            if ans >= n {
                return ans;
            }
        }

        unreachable!()
    }
}
