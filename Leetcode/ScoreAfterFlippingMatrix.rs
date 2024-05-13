// https://leetcode.com/problems/score-after-flipping-matrix

struct Solution {}

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        for i in 0..n {
            if grid[i][0] == 0 {
                for k in 0..m {
                    grid[i][k] ^= 1;
                }
            }
        }

        for j in 1..m {
            let mut s = 0;
            for i in 0..n {
                s += grid[i][j] as usize;
            }
            if s < n / 2 + n % 2 {
                for i in 0..n {
                    grid[i][j] ^= 1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            let mut p = 1 << (m - 1);
            let mut cur = 0;
            for j in 0..m {
                cur += grid[i][j] * p;
                p >>= 1;
            }
            ans += cur;
        }

        ans
    }
}
