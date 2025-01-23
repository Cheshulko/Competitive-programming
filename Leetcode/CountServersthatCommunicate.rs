// https://leetcode.com/problems/count-servers-that-communicate

struct Solution {}

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut rows = vec![0; n];
        let mut cols = vec![0; m];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 && (rows[i] > 1 || cols[j] > 1) {
                    ans += 1;
                }
            }
        }

        ans
    }
}
