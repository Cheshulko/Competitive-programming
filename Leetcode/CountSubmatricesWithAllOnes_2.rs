// https://leetcode.com/problems/count-submatrices-with-all-ones

struct Solution {}

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();

        let mut suf = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                let mut cnt = 0;
                for toj in j..m {
                    if mat[i][toj] == 1 {
                        cnt += 1;
                    } else {
                        break;
                    }
                }

                suf[i][j] = cnt;
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                let mut mi = i32::MAX;

                for toi in (0..=i).rev() {
                    mi = mi.min(suf[toi][j]);
                    if mi == 0 {
                        break;
                    }

                    ans += mi;
                }
            }
        }

        ans
    }
}
