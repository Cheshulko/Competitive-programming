// https://leetcode.com/problems/diagonal-traverse-ii

struct Solution {}

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(j, x)| (i + j, (-(i as i32), x)))
            })
            .collect::<Vec<_>>();

        nums.sort();
        nums.into_iter().map(|(_, (__, x))| x).collect()
    }
}
