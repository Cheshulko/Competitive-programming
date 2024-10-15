// https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers

struct Solution {}

impl Solution {
    const MOD: usize = 1_000_000_000 + 7;

    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        let (n, x) = (n as usize, x as usize);

        let mut nums = vec![];
        for i in 1..=n {
            let mut ii = 1;
            for _ in 0..x {
                ii *= i;
            }
            nums.push(ii);
        }

        let mut dp = vec![vec![0; nums.len() + 1]; n + 1];
        for j in 0..=nums.len() {
            dp[0][j] = 1;
        }
        for i in 1..=n {
            for j in 1..=nums.len() {
                dp[i][j] += dp[i][j - 1];
                dp[i][j] %= Solution::MOD;
                if i >= nums[j - 1] {
                    dp[i][j] += dp[i - nums[j - 1]][j - 1];
                    dp[i][j] %= Solution::MOD;
                }
            }
        }

        dp[n][nums.len()] as i32
    }
}
