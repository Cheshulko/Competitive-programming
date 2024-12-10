// https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-i

struct Solution {}

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let s = s
            .into_bytes()
            .into_iter()
            .map(|c| (c - b'a') as usize)
            .collect::<Vec<_>>();
        let n = s.len();

        let mut l = vec![vec![]; 26];
        let mut i = 0;

        while i < n {
            let cur = s[i];
            let mut cnt = 0;

            while i < n && s[i] == cur {
                cnt += 1;
                i += 1;
            }

            l[cur].push(cnt);
        }

        let mut ans = -1;
        for c in 0..26 {
            l[c].sort();
            l[c].reverse();

            if !l[c].is_empty() {
                if l[c][0] > 2 {
                    ans = ans.max(l[c][0] - 2);
                }
                if l[c].len() >= 3 {
                    ans = ans.max(l[c][2]);
                }
                if l[c].len() >= 2 {
                    if l[c][0] > l[c][1] {
                        ans = ans.max(l[c][1]);
                    } else {
                        ans = ans.max(l[c][1] - 1);
                    }
                }
            }
        }

        if ans > 0 {
            ans
        } else {
            -1
        }
    }
}
