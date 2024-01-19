// https://leetcode.com/problems/minimum-falling-path-sum

struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut dp = vec![vec![i32::MAX; m]; n];

        for j in 0..m {
            dp[0][j] = matrix[0][j];
        }

        for i in 0..n {
            for j in 0..m {
                for (value, to_i, to_j) in
                    [(1, -1), (1, 0), (1, 1)]
                        .into_iter()
                        .filter_map(|(di, dj)| {
                            let to_i = (i as i32 + di) as usize;
                            let to_j = (j as i32 + dj) as usize;

                            let value = matrix.get(to_i)?.get(to_j)?;
                            Some((value, to_i, to_j))
                        })
                {
                    dp[to_i][to_j] = dp[to_i][to_j].min(dp[i][j] + value);
                }
            }
        }

        *dp[n - 1].iter().min().unwrap_or(&i32::MAX)
    }
}
