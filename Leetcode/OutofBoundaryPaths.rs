// https://leetcode.com/problems/out-of-boundary-paths

struct Solution {}

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let (m, n, max_move, start_row, start_column) = (
            m as usize,
            n as usize,
            max_move as usize,
            start_row as usize,
            start_column as usize,
        );

        const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
        const MOD: i64 = 1_000_000_000 + 7;

        let is_valid = |x: i32, max: i32| x >= 0 && x < max;

        let mut dp = vec![vec![vec![0; max_move + 1]; n + 1]; m + 1];
        dp[start_row][start_column][max_move] = 1;

        let mut ans = 0;
        for moves in (1..=max_move).rev() {
            for i in 0..m {
                for j in 0..n {
                    let valids = DIRS
                        .iter()
                        .filter_map(|(di, dj)| {
                            let to_i = i as i32 + di;
                            let to_j = j as i32 + dj;

                            (is_valid(to_i, m as i32) && is_valid(to_j, n as i32))
                                .then_some((to_i as usize, to_j as usize))
                        })
                        .collect::<Vec<_>>();

                    ans += dp[i][j][moves] * (DIRS.len() - valids.len()) as i64;
                    ans %= MOD;

                    for (to_i, to_j) in valids.into_iter() {
                        dp[to_i][to_j][moves - 1] += dp[i][j][moves];
                        dp[to_i][to_j][moves - 1] %= MOD;
                    }
                }
            }
        }

        ans as i32
    }
}
