// https://leetcode.com/problems/largest-magic-square

struct Solution;

impl Solution {
    fn check(grid: &[Vec<i32>], sz: usize, si: usize, sj: usize) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        if si + sz > n || sj + sz > m {
            return false;
        }

        use std::collections::HashSet;

        let mut unique = HashSet::new();
        {
            for i in si..si + sz {
                let mut sum = 0;
                for j in sj..sj + sz {
                    sum += grid[i][j];
                }
                unique.insert(sum);
            }
        }
        {
            for j in sj..sj + sz {
                let mut sum = 0;
                for i in si..si + sz {
                    sum += grid[i][j];
                }
                unique.insert(sum);
            }
        }
        {
            let mut sum = 0;
            for d in 0..sz {
                sum += grid[si + d][sj + d];
            }
            unique.insert(sum);
        }
        {
            let mut sum = 0;
            for d in 0..sz {
                sum += grid[si + d][sj + sz - 1 - d];
            }
            unique.insert(sum);
        }

        unique.len() == 1
    }

    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        for sz in (1..=n.min(m)).rev() {
            for i in 0..n {
                for j in 0..m {
                    if Solution::check(&grid, sz, i, j) {
                        return sz as i32;
                    }
                }
            }
        }

        1
    }
}
