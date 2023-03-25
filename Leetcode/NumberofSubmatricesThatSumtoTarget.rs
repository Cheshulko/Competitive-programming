// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target

use std::collections::HashMap;

struct Solution {}

impl Solution {
    const MAX: usize = 101;

    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut colums_sum_window_cnt: Vec<Vec<HashMap<i64, i32>>> =
            vec![vec![HashMap::new(); Solution::MAX]; Solution::MAX];
        let mut rows_sum: Vec<Vec<i64>> = vec![vec![]; Solution::MAX];

        matrix.iter().enumerate().for_each(|(row_index, row)| {
            rows_sum[row_index].resize(row.len() + 1, 0);
            rows_sum[row_index][1] = row[0] as i64;
            row.iter()
                .enumerate()
                .skip(1)
                .for_each(|(column_index, value)| {
                    rows_sum[row_index][column_index + 1] =
                        rows_sum[row_index][column_index - 1 + 1] + *value as i64;
                });
        });

        // by column
        for i in 0..matrix[0].len() {
            // by column
            for j in i..matrix[0].len() {
                let mut pref_window_sum = 0;
                // by row
                for k in 0..matrix.len() {
                    let pref_k_window = rows_sum[k][j + 1] - rows_sum[k][i];

                    pref_window_sum += pref_k_window;

                    if pref_window_sum == target as i64 {
                        ans += 1;
                    }

                    let need_pref = pref_window_sum - target as i64;
                    let have_needed_pref: &i32 =
                        colums_sum_window_cnt[i][j].get(&need_pref).unwrap_or(&0) as &i32;
                    ans += have_needed_pref;

                    let value = colums_sum_window_cnt[i][j]
                        .get(&pref_window_sum)
                        .unwrap_or(&0)
                        .clone();
                    colums_sum_window_cnt[i][j].insert(pref_window_sum, value + 1);
                }
            }
        }
        ans
    }
}
