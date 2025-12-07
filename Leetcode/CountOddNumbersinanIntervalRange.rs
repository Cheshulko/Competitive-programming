// https://leetcode.com/problems/count-odd-numbers-in-an-interval-range

struct Solution {}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (1 + high - low + (high % 2)) / 2
    }
}
