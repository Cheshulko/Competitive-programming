// https://leetcode.com/problems/minimum-absolute-difference

struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();

        let d = arr.windows(2).map(|w| w[1] - w[0]).min().unwrap();

        arr.windows(2)
            .filter_map(|w| (w[1] - w[0] == d).then_some(vec![w[0], w[1]]))
            .collect()
    }
}
