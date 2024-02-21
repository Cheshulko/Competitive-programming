// https://leetcode.com/problems/bitwise-and-of-numbers-range

struct Solution {}

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut left = left as i64;
        let mut right = right as i64;
        let mut bit = 32;

        let mut ans = 0;
        while bit - 1 >= 0 {
            let low = 1 << (bit - 1);
            let high = 1 << bit;

            if left <= right && left >= low && right < high {
                left &= !low;
                right &= !low;

                ans |= low;
            }

            bit -= 1;
        }

        ans as i32
    }
}
