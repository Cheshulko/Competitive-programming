// https://leetcode.com/problems/find-the-highest-altitude

struct Solution {}

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.into_iter()
            .fold((0, 0), |(cur, ans), g| (cur + g, ans.max(cur + g)))
            .1
    }
}
