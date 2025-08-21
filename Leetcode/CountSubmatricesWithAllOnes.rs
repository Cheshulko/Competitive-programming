// https://leetcode.com/problems/count-submatrices-with-all-ones

mod cm_pref_sum_2 {
    // [ )
    pub fn get(sum: &Vec<Vec<i32>>, (i0, j0): (usize, usize), (i1, j1): (usize, usize)) -> i32 {
        sum[i1][j1] + sum[i0][j0] - sum[i0][j1] - sum[i1][j0]
    }

    pub fn build(arr: &Vec<Vec<i32>>, n: usize, m: usize) -> Vec<Vec<i32>> {
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

struct Solution {}

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let sum = cm_pref_sum_2::build(&mat, n, m);

        let mut ans = 0;
        for i1 in 0..n {
            for j1 in 0..m {
                for i0 in 0..=i1 {
                    for j0 in 0..=j1 {
                        let s1 = cm_pref_sum_2::get(&sum, (i0, j0), (i1 + 1, j1 + 1));
                        let s2 = ((i1 - i0 + 1) * (j1 - j0 + 1)) as i32;

                        ans += (s1 == s2) as i32;
                    }
                }
            }
        }

        ans
    }
}
