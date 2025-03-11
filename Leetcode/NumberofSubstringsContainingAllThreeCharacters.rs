// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters

struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let s = s
            .into_bytes()
            .into_iter()
            .map(|c| (c - b'a') as usize)
            .collect::<Vec<_>>();

        let mut ans = 0;
        let mut l = 0;
        let mut have = [0; 3];
        let mut have_cnt = 0;
        for r in 0..n {
            if have[s[r]] == 0 {
                have_cnt += 1;
            }
            have[s[r]] += 1;

            while l <= r && have[s[l]] > 1 {
                have[s[l]] -= 1;
                l += 1;
            }

            if have_cnt == 3 {
                ans += l + 1;
            }
        }

        ans as i32
    }
}
