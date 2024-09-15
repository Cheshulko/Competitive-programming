// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let lat = [b'a', b'e', b'i', b'o', b'u'];
        let v = lat
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<u8, usize>>();
        let s = s.into_bytes();
        let n = s.len();

        let mut dp = vec![[i32::MIN; 1 << 5]; n + 1];
        dp[0][0] = 0;

        let mut ans = 0;
        for i in 0..n {
            if let Some(&ind) = v.get(&s[i]) {
                for mask in 0..(1 << 5) {
                    dp[i + 1][1 << ind] = dp[i + 1][1 << ind].max(1);
                    if dp[i][mask ^ (1 << ind)] != i32::MIN {
                        dp[i + 1][mask] = dp[i][mask ^ (1 << ind)] + 1;
                    }
                }
            } else {
                dp[i + 1][0] = dp[i + 1][0].max(1);
                for mask in 0..(1 << 5) {
                    if dp[i][mask] != i32::MIN {
                        dp[i + 1][mask] = dp[i][mask] + 1;
                    }
                }
            }

            ans = ans.max(dp[i + 1][0]);
        }

        ans
    }
}
