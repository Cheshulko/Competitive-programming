// https://leetcode.com/problems/longest-palindrome

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnt = vec![0; 256];
        for c in s.into_bytes() {
            cnt[c as usize] += 1;
        }

        let mut ans = 0;
        let mut x = false;
        for i in 0..256 {
            ans += 2 * (cnt[i] / 2);
            if cnt[i] % 2 == 1 && !x {
                ans += 1;
                x = true;
            }
        }

        ans
    }
}
