// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix

struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len() as i32;

        let mut ans = 0;
        let mut j = m as i32 - 1;
        for i in 0..n {
            while j >= 0 && grid[i][j as usize] < 0 {
                j -= 1;
            }
            ans += m - j - 1
        }
        ans
    }
}
