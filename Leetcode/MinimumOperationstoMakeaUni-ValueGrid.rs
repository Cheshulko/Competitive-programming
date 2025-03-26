// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid

struct Solution {}

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut arr = grid.into_iter().flatten().collect::<Vec<_>>();
        arr.sort_unstable();

        let rem = arr[0] % x;
        if arr.iter().any(|&y| y % x != rem) {
            return -1;
        }

        let y = arr[arr.len() / 2];
        arr.iter().map(|&a| (a - y).abs() / x).sum::<i32>()
    }
}
