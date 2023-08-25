// https://leetcode.com/problems/interleaving-string

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn dfs(
        dp: &mut Vec<Vec<Option<bool>>>,
        ind: usize,
        target: &[u8],
        s1: &[u8],
        s2: &[u8],
    ) -> bool {
        if let Some(can) = dp[s1.len()][s2.len()] {
            return can;
        }

        match target.len().cmp(&ind) {
            Ordering::Equal if s1.is_empty() && s2.is_empty() => true,
            Ordering::Equal => false,
            _ => {
                dp[s1.len()][s2.len()] = Some(
                    (!s1.is_empty()
                        && target[ind] == s1[0]
                        && Solution::dfs(dp, ind + 1, target, &s1[1..], s2))
                        || (!s2.is_empty()
                            && target[ind] == s2[0]
                            && Solution::dfs(dp, ind + 1, target, s1, &s2[1..])),
                );

                dp[s1.len()][s2.len()].unwrap()
            }
        }
    }

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.chars().map(|c| c as u8).collect::<Vec<_>>();
        let s2 = s2.chars().map(|c| c as u8).collect::<Vec<_>>();
        let s3 = s3.chars().map(|c| c as u8).collect::<Vec<_>>();

        let mut dp = vec![vec![None; s2.len() + 1]; s1.len() + 1];

        Solution::dfs(&mut dp, 0, &s3, &s1, &s2)
    }
}
