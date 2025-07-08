// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii

struct Solution {}

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::Ordering;

        let n = events.len();

        events.sort_unstable_by(|a, b| {
            let (sa, ea) = (a[0], a[1]);
            let (sb, eb) = (b[0], b[1]);

            match ea.cmp(&eb) {
                Ordering::Equal => sa.cmp(&sb),
                r => r,
            }
        });

        let k = k as usize;

        // end, prev ind, sum
        let mut dp = vec![vec![(i32::MIN, 0, 0); n + 1]; k + 1];
        for i in 0..=n {
            dp[0][i] = (-1, 0, 0);
        }

        for cur_k in 1..=k {
            for (i, e) in events.iter().enumerate() {
                let i = i + 1;

                let (s, e, v) = (e[0], e[1], e[2]);

                let prev = dp[cur_k][i - 1];
                if prev.2 > dp[cur_k][i].2 {
                    dp[cur_k][i] = prev;
                }

                let j = dp[cur_k - 1].partition_point(|x| x.0 < s) - 1;
                let prev_valid = dp[cur_k - 1][j];

                if prev_valid.0 == i32::MIN {
                    continue;
                }

                if dp[cur_k][i].2 < prev_valid.2 + v {
                    dp[cur_k][i] = (e, j, prev_valid.2 + v);
                }
            }
        }

        (1..=k).map(|k| dp[k][n].2).max().unwrap()
    }
}
