// https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-ii

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn min_flips(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut ans = 0;
        for i in 0..n / 2 {
            for j in 0..m / 2 {
                let row_op = m - 1 - j;
                let col_op = n - 1 - i;

                let dif = [(i, j), (col_op, j), (i, row_op), (col_op, row_op)]
                    .into_iter()
                    .collect::<HashSet<_>>();
                let dl = dif.len();

                if dl == 4 {
                    let mut cnt = 0;
                    for (x, y) in dif.into_iter() {
                        cnt += grid[x][y];
                    }
                    ans += (4 - cnt).min(cnt);
                }
            }
        }

        if m % 2 == 1 && n % 2 == 1 {
            if grid[n / 2][m / 2] == 1 {
                ans += 1;
                grid[n / 2][m / 2] = 0;
            }
        }

        let mut ones = 0;
        let mut difs = 0;
        if m % 2 == 1 {
            let j = m / 2;
            for i in 0..n {
                ones += (grid[i][j] == 1) as i32;
            }
            for i in 0..n / 2 {
                difs += (grid[i][j] != grid[n - 1 - i][j]) as i32;
            }
        }

        if n % 2 == 1 {
            let i = n / 2;
            for j in 0..m {
                ones += (grid[i][j] == 1) as i32;
            }
            for j in 0..m / 2 {
                difs += (grid[i][j] != grid[i][m - 1 - j]) as i32;
            }
        }

        let mut to_add = (ones % 4).max(difs);
        if (ones + difs) % 4 == 0 {
            to_add = to_add.min(difs);
        }
        if (ones - difs) % 4 == 0 {
            to_add = to_add.min(difs);
        }

        ans + to_add
    }
}
