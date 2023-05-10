// https://leetcode.com/problems/spiral-matrix-ii

struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)];

        let mut ans: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];

        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut cur_dir_ind = 0;
        let mut cur = 1;

        let can = |i: i32, j: i32| i >= 0 && i < n && j >= 0 && j < n;

        loop {
            ans[i as usize][j as usize] = cur;
            cur += 1;

            let mut tries = 0;
            let (mut di, mut dj) = dirs[cur_dir_ind];

            while tries < dirs.len()
                && !(can(i + di, j + dj) && ans[(i + di) as usize][(j + dj) as usize] == 0)
            {
                tries += 1;
                cur_dir_ind = (cur_dir_ind + 1) % dirs.len();
                di = dirs[cur_dir_ind].0;
                dj = dirs[cur_dir_ind].1;
            }
            if tries >= dirs.len() {
                break;
            } else {
                i += di;
                j += dj;
            }
        }

        ans
    }
}
