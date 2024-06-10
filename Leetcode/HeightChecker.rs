// https://leetcode.com/problems/height-checker

struct Solution {}

impl Solution {
    pub fn height_checker(mut heights: Vec<i32>) -> i32 {
        let mut w = heights.clone();
        w.sort();

        heights
            .into_iter()
            .zip(w.into_iter())
            .filter(|(i, j)| i != j)
            .count() as i32
    }
}
