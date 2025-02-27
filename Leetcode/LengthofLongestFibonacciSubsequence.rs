// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let arr = arr.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let mut dp = vec![HashMap::<i64, usize>::new(); n];
        for i in 0..n {
            for j in 0..i {
                dp[i].insert(arr[j], 2);
            }
        }

        let mut ans = 0;
        for i in 0..n {
            let mut best = 0;
            for j in 0..i {
                let d = arr[i] - arr[j];

                let cur = *dp[j].entry(d).or_default();

                let upd = dp[i].entry(arr[j]).or_default();
                *upd = (*upd).max(cur + 1);

                best = best.max(*upd);
            }

            ans = ans.max(best);
        }

        if ans > 2 {
            ans as i32
        } else {
            0
        }
    }
}
