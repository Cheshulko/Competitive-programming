// https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/description

struct Solution {}

impl Solution {
    pub fn kth_character(k: i32) -> char {
        fn solve(k: i64, p: i64) -> i64 {
            if p == 1 {
                return 0;
            } else {
                if k < p / 2 {
                    return solve(k, p / 2);
                } else {
                    return 1 + solve(k - p / 2, p / 2);
                }
            }
        }

        let k = (k - 1) as i64;
        let mut p = 1;
        while p * 2 <= k {
            p *= 2;
        }

        let ans = solve(k, p * 2);
        (b'a' + (ans % 26) as u8) as char
    }
}
