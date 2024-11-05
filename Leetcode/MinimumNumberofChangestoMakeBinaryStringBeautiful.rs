// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful

struct Solution {}

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut ans = 0;

        let s = s.into_bytes();
        let n = s.len();

        let mut i = 0;
        let mut cnt = 0;
        while i < n {
            let cur = s[i];
            while i < n && s[i] == cur {
                cnt += 1;
                i += 1;
            }
            ans += cnt % 2;
            cnt = cnt % 2;
        }

        ans
    }
}
