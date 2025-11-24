// https://leetcode.com/problems/binary-prefix-divisible-by-5

struct Solution {}

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        nums.into_iter()
            .scan(0, |num, b| {
                *num <<= 1;
                *num %= 10;
                *num += b;

                Some(*num % 5 == 0)
            })
            .collect()
    }
}
