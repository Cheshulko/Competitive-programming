//  https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned = banned.into_iter().collect::<HashSet<_>>();

        let mut ans = 0;
        let mut sum = 0;
        let mut cur = 1;
        loop {
            if banned.contains(&cur) {
                cur += 1;
            } else {
                if sum + cur <= max_sum && cur <= n {
                    ans += 1;
                    sum += cur;
                    cur += 1;
                } else {
                    break;
                }
            }
        }

        ans
    }
}
