// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-i

struct Solution {}

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let (mut i0, mut i1) = (usize::MAX, usize::MIN);
        let (mut j0, mut j1) = (usize::MAX, usize::MIN);

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    i0 = i0.min(i);
                    j0 = j0.min(j);
                    i1 = i1.max(i);
                    j1 = j1.max(j);
                }
            }
        }

        ((i1 - i0 + 1) * (j1 - j0 + 1)) as i32
    }
}
