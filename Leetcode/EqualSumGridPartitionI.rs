// https://leetcode.com/problems/equal-sum-grid-partition-i

struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let grid = grid
            .into_iter()
            .map(|row| row.into_iter().map(i64::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let s = grid.iter().map(|row| row.iter().sum::<i64>()).sum::<i64>();

        let n = grid.len();
        let m = grid[0].len();

        {
            let mut s1 = 0;
            for i in 0..n {
                for j in 0..m {
                    s1 += grid[i][j];
                }

                if 2 * s1 == s {
                    return true;
                }
            }
        }
        {
            let mut s1 = 0;
            for j in 0..m {
                for i in 0..n {
                    s1 += grid[i][j];
                }

                if 2 * s1 == s {
                    return true;
                }
            }
        }

        false
    }
}
