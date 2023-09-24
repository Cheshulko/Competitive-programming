// https://leetcode.com/problems/champagne-tower

struct Solution {}

impl Solution {
    const MAX: usize = 100 + 1;

    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let query_row = query_row as usize;
        let query_glass = query_glass as usize;
        let poured = poured as f64;

        let mut dp = vec![vec![0.; Solution::MAX]; Solution::MAX];

        dp[0][0] = poured;

        for row in 0..query_row {
            for glass in 0..=row {
                let before = dp[row][glass];
                dp[row][glass] = before.min(1.);
                let extra = before - dp[row][glass];

                if extra > 0. {
                    for ind in glass..=(glass + 1) {
                        dp[row + 1][ind] += 0.5 * extra;
                    }
                }
            }
        }

        dp[query_row][query_glass].min(1.)
    }
}
