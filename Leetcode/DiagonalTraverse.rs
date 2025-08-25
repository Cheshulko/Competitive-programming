// https://leetcode.com/problems/diagonal-traverse

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let n = mat.len() as i32;
        let m = mat[0].len() as i32;

        let mut ans = vec![];

        const DIRS: &[(i32, i32)] = &[(-1, 1), (1, -1)];
        const FIXES: &[(i32, i32)] = &[(1, 0), (0, 1)];
        let mut d = 0;

        let (mut i, mut j) = (0, 0);
        let ma = n.max(m);
        for _ in 0..=2 * ma * ma {
            if i < n && j < m {
                ans.push(mat[i as usize][j as usize]);
            }

            i += DIRS[d].0;
            j += DIRS[d].1;

            if i < 0 || j < 0 {
                i += FIXES[d].0;
                j += FIXES[d].1;

                d = 1 - d;
            }
        }

        ans
    }
}
