// https://leetcode.com/problems/special-array-i

struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|v| v[0] & 1 != v[1] & 1)
    }
}
