// https://leetcode.com/problems/making-a-large-island

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

        let n = grid.len();

        fn dfs_cnt(cur: (usize, usize), grid: &Vec<Vec<i32>>, used: &mut Vec<Vec<i32>>) -> i32 {
            let (i, j) = cur;
            used[i][j] = 1;

            let mut cnt = 0;
            for to @ (to_i, to_j) in DIRS.iter().filter_map(|(di, dj)| {
                let to_i = (i as i32 + di) as usize;
                let to_j = (j as i32 + dj) as usize;
                (grid.get(to_i)?.get(to_j)? == &1).then_some((to_i, to_j))
            }) {
                if used[to_i][to_j] == 0 {
                    cnt += dfs_cnt(to, grid, used);
                }
            }

            1 + cnt
        }

        fn dfs_set(
            cur: (usize, usize),
            ty: usize,
            cnt: i32,
            grid: &Vec<Vec<i32>>,
            used: &mut Vec<Vec<i32>>,
            islands: &mut Vec<Vec<(i32, usize)>>,
        ) {
            let (i, j) = cur;
            used[i][j] = 2;
            islands[i][j] = (cnt, ty);

            for to @ (to_i, to_j) in DIRS.iter().filter_map(|(di, dj)| {
                let to_i = (i as i32 + di) as usize;
                let to_j = (j as i32 + dj) as usize;
                (grid.get(to_i)?.get(to_j)? == &1).then_some((to_i, to_j))
            }) {
                if used[to_i][to_j] == 1 {
                    dfs_set(to, ty, cnt, grid, used, islands);
                }
            }
        }

        let mut ty = 1;
        let mut ma = 0;
        let mut islands = vec![vec![(0, 0); n]; n];
        let mut used = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 && used[i][j] == 0 {
                    let cnt = dfs_cnt((i, j), &grid, &mut used);
                    dfs_set((i, j), ty, cnt, &grid, &mut used, &mut islands);
                    ty += 1;

                    ma = ma.max(cnt);
                }
            }
        }

        let mut ans = 0;
        let mut have_0 = false;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    have_0 = true;

                    let mut have = HashSet::new();
                    let mut s = 1;

                    for (to_i, to_j) in DIRS.iter().filter_map(|(di, dj)| {
                        let to_i = (i as i32 + di) as usize;
                        let to_j = (j as i32 + dj) as usize;
                        (grid.get(to_i)?.get(to_j)? == &1).then_some((to_i, to_j))
                    }) {
                        if grid[to_i][to_j] == 1 && !have.contains(&islands[to_i][to_j].1) {
                            s += islands[to_i][to_j].0;
                            have.insert(islands[to_i][to_j].1);
                        }
                    }

                    ans = ans.max(s);
                }
            }
        }

        if have_0 {
            ma += 1;
        }

        ans.max(ma)
    }
}
