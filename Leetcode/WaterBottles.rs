// https://leetcode.com/problems/water-bottles

struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        let mut ex = num_bottles;

        while ex >= num_exchange {
            let add = ex / num_exchange;
            ans += add;
            ex = add + ex % num_exchange;
        }

        ans
    }
}
