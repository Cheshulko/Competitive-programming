// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column

struct Solution {}

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();

        let mut onesRowi = vec![0; n];
        let mut onesColj = vec![0; m];
        let mut ans = vec![vec![-1 * (n + m) as i32; m]; n];

        for i in 0..n {
            for j in 0..m {
                onesRowi[i] += 2 * grid[i][j];
                onesColj[j] += 2 * grid[i][j];
            }
        }

        for i in 0..n {
            for j in 0..m {
                ans[i][j] += onesRowi[i] + onesColj[j];
            }
        }

        ans
    }
}
