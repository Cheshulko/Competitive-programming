// https://leetcode.com/problems/unique-length-3-palindromic-subsequences

struct Solution {}

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        const A: usize = 26;

        let s = s.into_bytes();

        let mut left = [usize::MAX; A];
        let mut right = [usize::MIN; A];
        let mut seen = [[false; A]; A];

        for (i, &c) in s.iter().enumerate() {
            let c = (c - b'a') as usize;

            left[c] = left[c].min(i);
            right[c] = right[c].max(i);
        }

        let mut ans = 0;
        for (i, &c) in s.iter().enumerate() {
            let c = (c - b'a') as usize;

            for c2 in 0..26 {
                if seen[c2][c] {
                    continue;
                }

                if left[c2] < i && i < right[c2] {
                    ans += 1;
                    seen[c2][c] = true;
                }
            }
        }

        ans
    }
}
