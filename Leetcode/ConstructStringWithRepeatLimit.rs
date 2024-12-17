// https://leetcode.com/problems/construct-string-with-repeat-limit

struct Solution {}

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        fn normalize(s: &mut Vec<char>, lim: usize) {
            let n = s.len();
            let mut i = 0;
            let mut j = 0;

            while i < n {
                let mut cnt = 1;
                let cur = s[i];
                while i < n && s[i] == cur {
                    i += 1;
                    cnt += 1;
                    if cnt == lim + 1 {
                        break;
                    }
                }
                if i < n && cnt == lim + 1 {
                    j = j.max(i);
                    while j < n && s[j] == cur {
                        j += 1;
                    }
                    if j < n && s[j] != cur {
                        s.swap(i, j);
                    }
                }
            }
        }

        let mut s = s.chars().collect::<Vec<_>>();
        s.sort_unstable();

        s.reverse();
        normalize(&mut s, repeat_limit as usize);

        let mut ans = vec![];
        let mut cnt = 0;
        let mut prev = 'A';
        for i in 0..s.len() {
            if s[i] != prev {
                cnt = 1;
                prev = s[i];
                ans.push(s[i]);
            } else {
                if cnt == repeat_limit {
                } else {
                    ans.push(s[i]);
                    cnt += 1;
                }
            }
        }

        ans.into_iter().collect()
    }
}
