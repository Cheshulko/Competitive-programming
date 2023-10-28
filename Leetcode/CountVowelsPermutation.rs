// https://leetcode.com/problems/count-vowels-permutation

struct Solution {}

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        /*
        a 0
        e 1
        i 2
        o 3
        u 4
        */
        const CNT: usize = 5;
        const MOD: i64 = 1_000_000_000 + 7;
        let n = n as usize;
        let mut can_follow = [[false; CNT]; CNT];
        can_follow[0][1] = true;

        can_follow[1][0] = true;
        can_follow[1][2] = true;

        can_follow[2][0] = true;
        can_follow[2][1] = true;
        can_follow[2][3] = true;
        can_follow[2][4] = true;

        can_follow[3][2] = true;
        can_follow[3][4] = true;

        can_follow[4][0] = true;

        let mut dp = vec![vec![0; CNT]; n + 1]; // dp[ind][last]
        let mut ans = 0;

        for c in 0..CNT {
            dp[1][c] = 1;
        }

        for i in 2..=n {
            for pc in 0..CNT {
                for c in 0..CNT {
                    if can_follow[c][pc] {
                        dp[i][c] = (dp[i][c] + dp[i - 1][pc]) % MOD;
                    }
                }
            }
        }

        for c in 0..CNT {
            ans = (ans + dp[n][c]) % MOD;
        }

        ans as i32
    }
}
