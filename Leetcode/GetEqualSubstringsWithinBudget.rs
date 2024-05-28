// https://leetcode.com/problems/get-equal-substrings-within-budget

struct Solution {}

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let st = s
            .into_iter()
            .zip(t.into_iter())
            .map(|(a, b)| a.abs_diff(b) as i32)
            .collect::<Vec<_>>();

        let n = st.len();
        let mut i = 0;
        let mut j = 1;
        let mut cur = st[i];
        let mut ans = if st[i] <= max_cost { 1 } else { 0 };

        while i < n && j < n {
            if cur + st[j] <= max_cost {
                cur += st[j];
                ans = ans.max(j - i + 1);
                j += 1;
            } else {
                cur -= st[i];
                i += 1;
            }
        }

        ans as i32
    }
}
