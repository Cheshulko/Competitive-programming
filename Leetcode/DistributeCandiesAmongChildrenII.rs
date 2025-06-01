// https://leetcode.com/problems/distribute-candies-among-children-ii

struct Solution {}

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let mut ans = 0;
        for first in 0..=limit.min(n) {
            let left = n - first;
            let variants = left + 1;
            let valid = (variants - 2 * (left - limit).max(0)).max(0);

            ans += valid as i64
        }

        ans
    }
}
