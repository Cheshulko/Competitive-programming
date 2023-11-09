// https://leetcode.com/problems/count-number-of-homogenous-substrings

struct Solution {}

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        const MOD: i32 = 1_000_000_000 + 7;
        let s = s.chars().map(|c| c as u8).collect::<Vec<_>>();
        let mut cur = s[0];
        let mut streak = 0;
        let mut streak_ans = 0;
        let mut ans = 0;
        for c in s.into_iter() {
            if c != cur {
                ans = (ans + streak_ans) % MOD;

                streak = 0;
                streak_ans = 0;
                cur = c;
            }
            streak += 1;
            streak_ans = (streak_ans + streak) % MOD;
        }
        ans = (ans + streak_ans) % MOD;
        ans
    }
}
