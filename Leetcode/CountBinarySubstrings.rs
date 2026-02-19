// https://leetcode.com/problems/count-binary-substrings

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut cnt = [0; 2];
        let mut ans = 0;
        let mut prev = 2;
        for c in s.into_bytes().into_iter() {
            let c = (c - b'0') as usize;

            if prev != c {
                cnt[c] = 0;
            }

            prev = c;
            cnt[c] += 1;
            ans += (cnt[c ^ 1] >= cnt[c]) as i32;
        }

        ans
    }
}
