// https://leetcode.com/problems/magic-squares-in-grid

struct Solution {}

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::BTreeSet;

        let n = grid.len();
        let m = grid[0].len();
        if n < 3 || m < 3 {
            return 0;
        }

        let mut ans = 0;
        for i in 0..n - 2 {
            for j in 0..m - 2 {
                let mut u = BTreeSet::new();
                let mut h = BTreeSet::new();

                for ii in i..i + 3 {
                    for jj in j..j + 3 {
                        u.insert(grid[ii][jj]);
                    }
                }

                h.insert(grid[i][j] + grid[i][j + 1] + grid[i][j + 2]);
                h.insert(grid[i + 1][j] + grid[i + 1][j + 1] + grid[i + 1][j + 2]);
                h.insert(grid[i + 2][j] + grid[i + 2][j + 1] + grid[i + 2][j + 2]);

                h.insert(grid[i][j] + grid[i + 1][j] + grid[i + 2][j]);
                h.insert(grid[i][j + 1] + grid[i + 1][j + 1] + grid[i + 2][j + 1]);
                h.insert(grid[i][j + 2] + grid[i + 1][j + 2] + grid[i + 2][j + 2]);

                h.insert(grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2]);
                h.insert(grid[i + 2][j] + grid[i + 1][j + 1] + grid[i][j + 2]);

                if h.len() == 1
                    && u.len() == 9
                    && *u.iter().last().unwrap() < 10
                    && *u.iter().next().unwrap() > 0
                {
                    ans += 1;
                }
            }
        }

        ans
    }
}
