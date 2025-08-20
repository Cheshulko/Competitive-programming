// https://leetcode.com/problems/count-square-submatrices-with-all-ones

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
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let sum = cm_pref_sum_2::build(&matrix, n, m);

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                for size in 0..=i.min(j) {
                    let x = cm_pref_sum_2::get(&sum, (i - size, j - size), (i + 1, j + 1));
                    let y = ((size + 1) * (size + 1)) as i32;
                    ans += (x == y) as i32;
                }
            }
        }

        ans
    }
}
