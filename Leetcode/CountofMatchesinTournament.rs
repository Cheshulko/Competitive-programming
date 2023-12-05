// https://leetcode.com/problems/count-of-matches-in-tournament

struct Solution {}

impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut ans = 0;
        while n != 1 {
            ans += n / 2;
            n = n / 2 + n % 2;
        }
        ans
    }
}
