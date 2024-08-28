// https://leetcode.com/problems/count-sub-islands

struct Solution {}

impl Solution {
    const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn dfs(grid: &mut Vec<Vec<i32>>, cur: (usize, usize), ind: i32) {
        grid[cur.0][cur.1] = ind;

        let x = Solution::DIRS
            .iter()
            .filter_map(|(di, dj)| {
                let to_i = (cur.0 as i32 + di) as usize;
                let to_j = (cur.1 as i32 + dj) as usize;
                (grid.get(to_i)?.get(to_j)? == &1).then_some((to_i, to_j))
            })
            .collect::<Vec<_>>();

        for (to_i, to_j) in x.into_iter() {
            Solution::dfs(grid, (to_i, to_j), ind);
        }
    }

    fn check(
        grid1: &Vec<Vec<i32>>,
        grid2: &mut Vec<Vec<i32>>,
        cur: (usize, usize),
        ind1: i32,
        ind2: i32,
    ) -> bool {
        grid2[cur.0][cur.1] = -1;

        let x = Solution::DIRS
            .iter()
            .filter_map(|(di, dj)| {
                let to_i = (cur.0 as i32 + di) as usize;
                let to_j = (cur.1 as i32 + dj) as usize;
                (grid2.get(to_i)?.get(to_j)? == &ind2).then_some((to_i, to_j))
            })
            .collect::<Vec<_>>();

        let mut ok = true;
        for (to_i, to_j) in x.into_iter() {
            if grid1[to_i][to_j] == ind1 {
                ok &= Solution::check(grid1, grid2, (to_i, to_j), ind1, ind2);
            } else {
                ok = false;
                Solution::check(grid1, grid2, (to_i, to_j), ind1, ind2);
            }
        }

        return ok;
    }

    pub fn count_sub_islands(mut grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        let n = grid1.len();
        let m = grid1[0].len();

        let mut ind = 2;
        for i in 0..n {
            for j in 0..m {
                if grid1[i][j] == 1 {
                    Solution::dfs(&mut grid1, (i, j), ind);
                    ind += 1;
                }
            }
        }

        let mut ind = 2;
        for i in 0..n {
            for j in 0..m {
                if grid2[i][j] == 1 {
                    Solution::dfs(&mut grid2, (i, j), ind);
                    ind += 1;
                }
            }
        }

        let mut cnt = 0;
        for i in 0..n {
            for j in 0..m {
                if grid1[i][j] != 0 && grid2[i][j] != 0 && grid2[i][j] != -1 {
                    let ind1 = grid1[i][j];
                    let ind2 = grid2[i][j];
                    cnt += Solution::check(&grid1, &mut grid2, (i, j), ind1, ind2) as i32;
                }
            }
        }

        cnt
    }
}
