// https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows

struct Solution {}

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut ans = 0;
        for i in 0..n {
            let mut flip = vec![0; m];
            for j in 1..m {
                flip[j] = matrix[i][j] ^ matrix[i][0];
            }

            let mut cur = 0;
            for ii in 0..n {
                let mut ok = true;
                for j in 1..m {
                    ok &= matrix[ii][j] ^ flip[j] == matrix[ii][0];
                }
                cur += ok as i32;
            }
            ans = ans.max(cur);
        }

        ans
    }
}
