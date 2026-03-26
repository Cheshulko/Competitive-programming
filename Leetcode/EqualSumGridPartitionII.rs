// https://leetcode.com/problems/equal-sum-grid-partition-ii

struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        use std::collections::{HashMap, HashSet};

        type Map = HashMap<i64, HashSet<(usize, usize)>>;

        let grid = grid
            .into_iter()
            .map(|row| row.into_iter().map(i64::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let s = grid.iter().map(|row| row.iter().sum::<i64>()).sum::<i64>();

        let n = grid.len();
        let m = grid[0].len();

        let mut positions_rest = Map::new();
        for i in 0..n {
            for j in 0..m {
                positions_rest.entry(grid[i][j]).or_default().insert((i, j));
            }
        }

        {
            let mut s1 = 0;
            let mut positions_cur = Map::new();
            let mut positions_rest = positions_rest.clone();

            for i in 0..n - 1 {
                for j in 0..m {
                    s1 += grid[i][j];
                    positions_cur.entry(grid[i][j]).or_default().insert((i, j));
                    positions_rest
                        .entry(grid[i][j])
                        .or_default()
                        .remove(&(i, j));
                }

                let s2 = s - s1;
                if s1 == s2 {
                    return true;
                }

                let dif = s2.abs_diff(s1) as i64;
                let positions = if s1 > s2 {
                    &positions_cur
                } else {
                    &positions_rest
                };

                if let Some(pps) = positions.get(&dif) {
                    for &(ii, jj) in pps {
                        if i == 0 && (jj > 0 && jj < m - 1) && ii == 0 {
                            continue;
                        }
                        if i == n - 2 && (jj > 0 && jj < m - 1) && ii == n - 1 {
                            continue;
                        }
                        if m == 1 && (ii != 0 && ii != i && ii != i + 1 && ii != n - 1) {
                            continue;
                        }

                        return true;
                    }
                }
            }
        }

        {
            let mut s1 = 0;
            let mut positions_cur = Map::new();
            let mut positions_rest = positions_rest.clone();

            for j in 0..m - 1 {
                for i in 0..n {
                    s1 += grid[i][j];
                    positions_cur.entry(grid[i][j]).or_default().insert((i, j));
                    positions_rest
                        .entry(grid[i][j])
                        .or_default()
                        .remove(&(i, j));
                }

                let s2 = s - s1;
                if s1 == s2 {
                    return true;
                }

                let dif = s2.abs_diff(s1) as i64;
                let positions = if s1 > s2 {
                    &positions_cur
                } else {
                    &positions_rest
                };

                if let Some(pps) = positions.get(&dif) {
                    for &(ii, jj) in pps {
                        if j == 0 && (ii > 0 && ii < n - 1) && jj == 0 {
                            continue;
                        }
                        if j == m - 2 && (ii > 0 && ii < n - 1) && jj == m - 1 {
                            continue;
                        }
                        if n == 1 && (jj != 0 && jj != j && jj != j + 1 && jj != m - 1) {
                            continue;
                        }

                        return true;
                    }
                }
            }
        }

        false
    }
}
