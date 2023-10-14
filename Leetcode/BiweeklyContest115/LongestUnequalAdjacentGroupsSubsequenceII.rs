impl Solution {
    pub fn get_words_in_longest_subsequence(
        n: i32,
        words: Vec<String>,
        groups: Vec<i32>,
    ) -> Vec<String> {
        let n = n as usize;

        let mut dp = vec![(-1, 0, -1); n]; // (par, mx, index cur)
        let mut dis = vec![vec![i32::MAX; n]; n];

        for i in 0..n {
            dp[i].2 = i as i32;
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let w1 = words[i].chars().map(|c| c as u8).collect::<Vec<_>>();
                let w2 = words[j].chars().map(|c| c as u8).collect::<Vec<_>>();

                let d1 = w1.len();
                let d2 = w2.len();

                if d1 != d2 {
                    continue;
                }

                let mut d = 0;

                for p in 0..d1 {
                    d += (w1[p] != w2[p]) as usize;
                }

                dis[i][j] = d as i32;
                dis[j][i] = d as i32;
            }
        }

        let mut mx = dp[0]; // (index par, mx,  index cur,)

        for i in 0..n {
            for j in (i + 1)..n {
                if dp[j].1 < dp[i].1 + 1 && groups[j] != groups[i] && dis[i][j] == 1 {
                    dp[j].1 = dp[i].1 + 1;
                    dp[j].0 = i as i32;
                }
                if dp[j].1 > mx.1 {
                    mx = dp[j];
                }
            }
        }

        let mut ans = vec![];

        loop {
            ans.push(words[mx.2 as usize].to_owned());
            if mx.0 == -1 {
                break;
            } else {
                mx = dp[mx.0 as usize];
            }
        }

        ans.into_iter().rev().collect::<Vec<_>>()
    }
}
