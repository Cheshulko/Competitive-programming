// https://leetcode.com/problems/delete-columns-to-make-sorted-iii

struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        fn sovle(col: usize, prev: usize, strs: &Vec<Vec<u8>>, dp: &mut Vec<Vec<i32>>) -> i32 {
            let n = strs.len();
            let m = strs[0].len();

            if col > m {
                return 0;
            }

            if dp[col][prev] != i32::MAX {
                return dp[col][prev];
            }

            let mut ok = true;
            if prev > 0 {
                for i in 0..n {
                    ok = ok && strs[i][col - 1] >= strs[i][prev - 1];
                }
            }

            if ok {
                dp[col][prev] = dp[col][prev].min(sovle(col + 1, col, strs, dp));
            }

            dp[col][prev] = dp[col][prev].min(1 + sovle(col + 1, prev, strs, dp));

            dp[col][prev]
        }

        let strs = strs.into_iter().map(|s| s.into_bytes()).collect::<Vec<_>>();

        let m = strs[0].len();
        let mut dp = vec![vec![i32::MAX; m + 1]; m + 1];

        sovle(1, 0, &strs, &mut dp)
    }
}
