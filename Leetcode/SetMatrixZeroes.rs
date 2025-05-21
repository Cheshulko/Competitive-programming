// https://leetcode.com/problems/set-matrix-zeroes

struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut x = 0;
        loop {
            let mut found = false;
            'out: for i in 0..n {
                for j in 0..m {
                    if matrix[i][j] == x {
                        found = true;
                        break 'out;
                    }
                }
            }

            if !found {
                break;
            } else {
                x += 1;
            }
        }

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    continue;
                }
                for i1 in 0..n {
                    if matrix[i1][j] == 0 {
                        matrix[i][j] = x;
                        break;
                    }
                }
                for j1 in 0..m {
                    if matrix[i][j1] == 0 {
                        matrix[i][j] = x;
                        break;
                    }
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == x {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
