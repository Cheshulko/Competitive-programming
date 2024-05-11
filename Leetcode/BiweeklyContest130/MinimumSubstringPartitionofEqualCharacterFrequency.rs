impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let n = s.len();
        let s = s
            .into_bytes()
            .into_iter()
            .map(|x| (x - b'a') as usize)
            .collect::<Vec<_>>();

        let mut cnt = vec![vec![0; 30]; n + 1];
        let mut ans = 0;

        for i in 1..=n {
            let x = s[i - 1];
            for c in 0..30 {
                cnt[i][c] = cnt[i - 1][c];
            }
            cnt[i][x] += 1;
        }

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            for j in 0..i {
                let mut pr = vec![0; 30];
                for c in 0..30 {
                    pr[c] = cnt[i][c] - cnt[j][c];
                }

                let mut ok = true;
                let mut g = 0;
                for c in 0..30 {
                    if pr[c] != 0 {
                        g = pr[c];
                        break;
                    }
                }

                for c in 0..30 {
                    if pr[c] != 0 && pr[c] != g {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    dp[i] = dp[i].min(dp[j] + 1);
                }
            }
        }

        dp[n]
    }
}
