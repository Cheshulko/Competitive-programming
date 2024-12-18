// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop

use std::cmp::Reverse;
use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();

        let mut prices_sorted = prices
            .iter()
            .enumerate()
            .map(|(i, x)| (x, Reverse(i)))
            .collect::<Vec<_>>();
        prices_sorted.sort_unstable();

        let mut bt = BTreeSet::<usize>::new();
        let mut ans = vec![0; n];
        for &(price, Reverse(i)) in prices_sorted.iter() {
            if let Some(&next) = bt.range(i..).next() {
                ans[i] = price - prices[next];
            } else {
                ans[i] = *price;
            }

            bt.insert(i);
        }

        ans
    }
}
