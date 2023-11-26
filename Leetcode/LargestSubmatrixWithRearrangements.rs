// https://leetcode.com/problems/largest-submatrix-with-rearrangements

/*
| in |streak|sort non-decr|max((i+1)*min(v[..=i]))|max rows|
|----|------|-------------|-----------------------|--------|
|0000| 0000 |    0000     |       0  0  0  0      |   0    |
|0110| 0430 |    4300     |       4  6  6  6      |   6    |
|0110| 0320 |    3200     |       3  4  4  4      |   6    |
|0110| 0210 |    2100     |       2  2  2  2      |   6    |
|1101| 5105 |    5510     |       5 10 10 10      |   10   |
|1001| 4004 |    4400     |       4  8  8  8      |   10   |
|1001| 3003 |    3300     |       3  3  3  3      |   10   |
|1101| 2202 |    2220     |       2  4  6  6      |   10   |
|1101| 1101 |    1110     |       1  2  3  3      |   10   |
*/

struct Solution {}

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut column_streaks = vec![vec![0; n]; m];

        for j in 0..m {
            for i in (0..n).rev() {
                column_streaks[j][i] =
                    matrix[i][j] * column_streaks[j].get(i + 1).unwrap_or(&0) + matrix[i][j];
            }
        }

        let mut rows_top_streaks = vec![];
        for i in 0..n {
            let mut row_top = vec![];
            for j in 0..m {
                row_top.push(column_streaks[j][i]);
            }
            row_top.sort_unstable_by(|a, b| b.cmp(&a));
            rows_top_streaks.push(row_top);
        }

        let mut ans = 0;
        for i in 0..n {
            let mut ans_ = 0;
            let mut mn = i32::MAX;
            for j in 0..m {
                mn = mn.min(rows_top_streaks[i][j]);
                ans_ = ans_.max(mn * (j + 1) as i32);
            }

            ans = ans.max(ans_);
        }

        ans
    }
}
