// https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating

struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let s = s.into_bytes();
        let n = s.len();

        let mut pref0 = vec![0; n + 1];
        let mut pref1 = vec![0; n + 1];
        for i in 0..n {
            pref0[i + 1] += pref0[i] + ([b'0', b'1'][i & 1] != s[i]) as i32;
            pref1[i + 1] += pref1[i] + ([b'1', b'0'][i & 1] != s[i]) as i32;
        }

        let mut suf0 = vec![0; n + 1];
        let mut suf1 = vec![0; n + 1];
        for i in (0..n).rev() {
            suf0[i] += suf0[i + 1] + ([b'0', b'1'][i & 1] != s[i]) as i32;
            suf1[i] += suf1[i + 1] + ([b'1', b'0'][i & 1] != s[i]) as i32;
        }

        let mut ans = pref0[n].min(pref1[n]);
        if (n & 1) == 1 {
            for i in 0..=n {
                ans = ans.min(pref0[i] + suf1[i]);
                ans = ans.min(pref1[i] + suf0[i]);
            }
        } else {
            for i in 0..=n {
                ans = ans.min(pref0[i] + suf0[i]);
                ans = ans.min(pref1[i] + suf1[i]);
            }
        }

        ans
    }
}
