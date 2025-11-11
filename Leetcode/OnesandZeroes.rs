// https://leetcode.com/problems/ones-and-zeroes

struct Solution {}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let strs = strs
            .into_iter()
            .map(|s| {
                s.into_bytes().into_iter().fold([0; 2], |mut v, b| {
                    v[(b - b'0') as usize] += 1;
                    v
                })
            })
            .collect::<Vec<_>>();

        let l = strs.len();

        let mut dp = vec![vec![vec![0; n + 1]; m + 1]; l + 1];
        let mut ans = 0;
        for k in 1..=l {
            let zz = strs[k - 1][0];
            let oo = strs[k - 1][1];
            if zz <= m && oo <= n {
                dp[k][zz][oo] = 1;
                ans = ans.max(1);
            }

            for z in 0..=m {
                for o in 0..=n {
                    dp[k][z][o] = dp[k][z][o].max(dp[k - 1][z][o]);
                    if z + zz <= m && o + oo <= n {
                        dp[k][z + zz][o + oo] = dp[k][z + zz][o + oo].max(dp[k - 1][z][o] + 1);
                        ans = ans.max(dp[k][z + zz][o + oo]);
                    }
                }
            }
        }

        ans
    }
}
