// https://leetcode.com/problems/stone-game-iii

struct Solution {}

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = vec![vec![vec![-1; 4]; n + 1]; 2]; // dp[t][i][m]

        fn go(
            t: usize,
            i: usize,
            m: usize,
            stone_value: &Vec<i32>,
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
                0 => (1..=(n - i).min(3)).fold((0, i32::MIN), |(s, res), x| {
                    (
                        s + stone_value[i + x - 1],
                        res.max(
                            s + stone_value[i + x - 1] + go(t ^ 1, i + x, 1, stone_value, dp, n),
                        )
                        .max(s + stone_value[i + x - 1] + go(t ^ 1, i + x, 2, stone_value, dp, n))
                        .max(s + stone_value[i + x - 1] + go(t ^ 1, i + x, 3, stone_value, dp, n)),
                    )
                }),
                1 => (1..=(n - i).min(3)).fold((0, i32::MAX), |(s, res), x| {
                    (
                        s + stone_value[i + x - 1],
                        res.min(go(t ^ 1, i + x, 1, stone_value, dp, n))
                            .min(go(t ^ 1, i + x, 2, stone_value, dp, n))
                            .min(go(t ^ 1, i + x, 3, stone_value, dp, n)),
                    )
                }),
                _ => unreachable!(),
            }
            .1;
            dp[t][i][m]
        }

        let x = go(0, 0, 1, &stone_value, &mut dp, &n);
        let s = stone_value.iter().sum::<i32>();

        match x.cmp(&(s - x)) {
            std::cmp::Ordering::Less => "Bob".to_string(),
            std::cmp::Ordering::Equal => "Tie".to_string(),
            std::cmp::Ordering::Greater => "Alice".to_string(),
        }
    }
}
