// https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer

struct Solution {}

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut y = 0;

        for n in nums.into_iter() {
            x += (n > 0) as i32;
            y += (n < 0) as i32;
        }

        x.max(y)
    }
}
