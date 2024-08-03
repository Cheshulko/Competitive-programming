// https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-i

struct Solution {}

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut rcnt = 0;
        for i in 0..n {
            let mut cnt = 0;
            for j in 0..m / 2 {
                if grid[i][j] != grid[i][m - 1 - j] {
                    cnt += 1;
                }
            }
            rcnt += cnt;
        }

        let mut ccnt = 0;
        for j in 0..m {
            let mut cnt = 0;
            for i in 0..n / 2 {
                if grid[i][j] != grid[n - 1 - i][j] {
                    cnt += 1;
                }
            }
            ccnt += cnt;
        }

        rcnt.min(ccnt)
    }
}
