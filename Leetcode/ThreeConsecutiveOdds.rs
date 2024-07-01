// https://leetcode.com/problems/three-consecutive-odds

struct Solution {}

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3).any(|v| v[0] & v[1] & v[2] & 1 > 0)
    }
}
