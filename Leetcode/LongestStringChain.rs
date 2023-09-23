// https://leetcode.com/problems/longest-string-chain

struct Solution {}

impl Solution {
    fn check(from: &str, to: &str) -> bool {
        let from = from.to_string();
        (0..to.len()).any(|p| from.to_owned() == format!("{}{}", &to[0..p], &to[(p + 1)..]))
    }

    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let n = words.len();
        let mut grid = vec![vec![]; n];

        words.sort_by(|a, b| a.len().cmp(&b.len()));

        for i in 0..n {
            for j in (i + 1)..n {
                let from = &words[i];
                let to = &words[j];

                if to.len() - from.len() > 1 {
                    break;
                }

                if to.len() - from.len() == 1 && Solution::check(from, to) {
                    grid[i].push(j);
                }
            }
        }

        let mut dp = vec![0; n];
        let mut ans = 0;

        for i in 0..n {
            for to in &grid[i] {
                let to = *to;
                dp[to] = dp[to].max(dp[i] + 1);
                ans = ans.max(dp[to]);
            }
        }

        ans + 1
    }
}
