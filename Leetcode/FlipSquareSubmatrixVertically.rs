// https://leetcode.com/problems/flip-square-submatrix-vertically

struct Solution;

impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;

        for i in x..x + k / 2 {
            for j in y..y + k {
                let i2 = x + k - (i - x) - 1;
                grid[i][j] ^= grid[i2][j];
                grid[i2][j] ^= grid[i][j];
                grid[i][j] ^= grid[i2][j];
            }
        }

        grid
    }
}
