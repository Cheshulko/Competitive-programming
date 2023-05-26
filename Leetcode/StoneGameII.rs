// https://leetcode.com/problems/stone-game-ii

struct Solution {}

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut dp = vec![vec![vec![-1; 2 * n + 1]; n + 1]; 2]; // dp[t][i][m]

        fn go(
            t: usize,
            i: usize,
            m: usize,
            piles: &Vec<i32>,
            dp: &mut Vec<Vec<Vec<i32>>>,
            n: &usize,
        ) -> i32 {
            if &i == n {
                return 0;
            }
            if dp[t][i][m] != -1 {
                return dp[t][i][m];
            }

            dp[t][i][m] = match t {
                0 => (1..=(n - i).min(2 * m)).fold((0, i32::MIN), |(s, res), x| {
                    (
                        s + piles[i + x - 1],
                        res.max(s + piles[i + x - 1] + go(t ^ 1, i + x, m.max(x), piles, dp, n)),
                    )
                }),
                1 => (1..=(n - i).min(2 * m)).fold((0, i32::MAX), |(s, res), x| {
                    (
                        s + piles[i + x - 1],
                        res.min(go(t ^ 1, i + x, m.max(x), piles, dp, n)),
                    )
                }),
                _ => unreachable!(),
            }
            .1;
            dp[t][i][m]
        }

        go(0, 0, 1, &piles, &mut dp, &n)
    }
}
