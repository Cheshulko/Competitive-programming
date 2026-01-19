// https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold

mod cm_pref_sum_2 {
    // [ )
    pub fn get(sum: &Vec<Vec<i64>>, (i0, j0): (usize, usize), (i1, j1): (usize, usize)) -> i64 {
        sum[i1][j1] + sum[i0][j0] - sum[i0][j1] - sum[i1][j0]
    }

    pub fn build(arr: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let n = arr.len();
        let m = arr[0].len();

        let mut sum = vec![vec![0; m + 1]; n + 1];

        for i in 0..n {
            for j in 0..m {
                sum[i + 1][j + 1] = arr[i][j];
                sum[i + 1][j + 1] += sum[i][j + 1];
                sum[i + 1][j + 1] += sum[i + 1][j];
                sum[i + 1][j + 1] -= sum[i][j];
            }
        }

        sum
    }
}

struct Solution;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let threshold = threshold as i64;
        let mat = mat
            .into_iter()
            .map(|r| r.into_iter().map(|x| x as i64).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let n = mat.len();
        let m = mat[0].len();

        let sum = cm_pref_sum_2::build(&mat);

        let mut ans = 0;
        for sz in 1..=n.min(m) {
            for i in 0..n {
                for j in 0..m {
                    if i + sz > n || j + sz > m {
                        continue;
                    }

                    if cm_pref_sum_2::get(&sum, (i, j), (i + sz, j + sz)) <= threshold {
                        ans = ans.max(sz);
                    }
                }
            }
        }

        ans as i32
    }
}
