// https://leetcode.com/problems/finding-3-digit-even-numbers

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();

        let mut nums = BTreeSet::new();
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if i == j || j == k || i == k {
                        continue;
                    }
                    if digits[k] & 1 == 1 || digits[i] == 0 {
                        continue;
                    }
                    nums.insert(digits[i] * 100 + digits[j] * 10 + digits[k]);
                }
            }
        }

        nums.into_iter().collect()
    }
}
