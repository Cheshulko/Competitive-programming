// https://leetcode.com/problems/water-bottles-ii

struct Solution {}

impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut ans = 0;
        let mut empty = 0;

        while num_bottles > 0 {
            ans += num_bottles;
            empty += num_bottles;
            num_bottles = 0;

            while empty >= num_exchange {
                empty -= num_exchange;
                num_bottles += 1;
                num_exchange += 1;
            }
        }

        ans
    }
}
