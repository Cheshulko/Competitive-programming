// https://leetcode.com/problems/take-k-of-each-character-from-left-and-right

struct Solution {}

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s = s
            .into_bytes()
            .into_iter()
            .map(|c| (c - b'a') as usize)
            .collect::<Vec<_>>();
        let n = s.len();
        let k = k as usize;

        let mut suf = vec![[usize::MAX; 3]; n + 1];
        suf[0][0] = 0;
        suf[0][1] = 0;
        suf[0][2] = 0;
        let mut cnt = [0; 3];
        for i in (0..n).rev() {
            cnt[s[i]] += 1;
            suf[cnt[s[i]]][s[i]] = n - i;
        }

        let mut ans = 0;
        for t in 0..3 {
            ans = ans.max(suf[k][t]);
        }

        let mut cnt = [0; 3];
        for i in 0..n {
            cnt[s[i]] += 1;
            cnt[s[i]] = cnt[s[i]].min(k);

            let mut need = 0;
            for t in 0..3 {
                if suf[k - cnt[t]][t] != usize::MAX && i + 1 + suf[k - cnt[t]][t] <= n {
                    need = need.max(i + 1 + suf[k - cnt[t]][t]);
                } else {
                    need = usize::MAX;
                }
            }
            ans = ans.min(need);
        }

        if ans != usize::MAX {
            ans as i32
        } else {
            -1
        }
    }
}
