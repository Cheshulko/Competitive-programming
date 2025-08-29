// https://leetcode.com/problems/alice-and-bob-playing-flower-game/description

struct Solution {}

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let (n, m) = (n as i64, m as i64);
        let mut ans = 0;

        for i in 1..=n {
            let x = 1 - i % 2;
            ans += m / 2 + x * (m % 2);
        }

        ans
    }
}
