// https://leetcode.com/problems/spiral-matrix

struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len() as i32;
        let m = matrix[0].len() as i32;

        let mut used: Vec<Vec<bool>> = vec![vec![false; m as usize]; n as usize];
        let dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)];
        let mut ans: Vec<i32> = vec![];

        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut cur_dir_ind = 0;

        let can = |i: i32, j: i32| i >= 0 && i < n && j >= 0 && j < m;

        loop {
            ans.push(matrix[i as usize][j as usize]);
            used[i as usize][j as usize] = true;

            let mut tries = 0;
            let (mut di, mut dj) = dirs[cur_dir_ind];

            while tries < dirs.len()
                && !(can(i + di, j + dj) && !used[(i + di) as usize][(j + dj) as usize])
            {
                tries += 1;
                cur_dir_ind = (cur_dir_ind + 1) % dirs.len();
                di = dirs[cur_dir_ind].0;
                dj = dirs[cur_dir_ind].1;
            }
            if tries >= 4 {
                break;
            } else {
                i += di;
                j += dj;
            }
        }

        ans
    }
}
