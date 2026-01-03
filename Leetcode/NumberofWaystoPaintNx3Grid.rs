// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid

struct Solution {}

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let n = n as usize;

        let mut dp = vec![[[[0; 3]; 3]; 3]; n];

        for c1 in 0..3 {
            for c2 in 0..3 {
                for c3 in 0..3 {
                    if c1 == c2 || c2 == c3 {
                        continue;
                    }

                    dp[0][c1][c2][c3] += 1;
                }
            }
        }

        for row in 1..n {
            for c1_up in 0..3 {
                for c2_up in 0..3 {
                    for c3_up in 0..3 {
                        if c1_up == c2_up || c2_up == c3_up {
                            continue;
                        }

                        for c1 in 0..3 {
                            for c2 in 0..3 {
                                for c3 in 0..3 {
                                    if c1 == c2 || c2 == c3 {
                                        continue;
                                    }

                                    if c1 == c1_up || c2 == c2_up || c3 == c3_up {
                                        continue;
                                    }

                                    dp[row][c1][c2][c3] += dp[row - 1][c1_up][c2_up][c3_up];
                                    dp[row][c1][c2][c3] %= M;
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut ans = 0;
        for c1 in 0..3 {
            for c2 in 0..3 {
                for c3 in 0..3 {
                    if c1 == c2 || c2 == c3 {
                        continue;
                    }

                    ans += dp[n - 1][c1][c2][c3];
                    ans %= M;
                }
            }
        }

        ans as i32
    }
}
