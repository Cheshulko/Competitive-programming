// https://leetcode.com/problems/maximum-number-of-coins-you-can-get

struct Solution {}

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let mut l = 0;
        let mut r = piles.len() - 1;

        let mut ans = 0;
        while l < r {
            ans += piles[r - 1];
            r -= 2;
            l += 1;
        }
        ans
    }
}
