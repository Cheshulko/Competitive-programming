// https://leetcode.com/problems/monotonic-array

struct Solution {}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let difs = nums.windows(2).map(|v| v[0] - v[1]).collect::<Vec<_>>();
        difs.iter().all(|x| x >= &0) || difs.iter().all(|x| x <= &0)
    }
}
