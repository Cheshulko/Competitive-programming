// https://leetcode.com/problems/maximum-length-of-pair-chain

struct Solution {}

impl Solution {
    fn dfs(dp: &mut Vec<i32>, g: &Vec<Vec<bool>>, ind: usize) -> i32 {
        if dp[ind] != -1 {
            return dp[ind];
        }

        let mut ans = 0;
        for i in 0..g.len() {
            if g[ind][i] {
                ans = ans.max(1 + Solution::dfs(dp, g, i))
            }
        }

        dp[ind] = ans;
        dp[ind]
    }

    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let n = pairs.len();

        let mut g = vec![vec![false; n]; n];
        let mut dp = vec![-1; n];

        for i in 0..n {
            for j in (i + 1)..n {
                g[i][j] = pairs[i][0] > pairs[j][1];
                g[j][i] = pairs[j][0] > pairs[i][1]
            }
        }

        let mut ans = 0;

        for i in 0..n {
            ans = ans.max(1 + Solution::dfs(&mut dp, &g, i));
        }

        ans
    }
}
