// https://leetcode.com/problems/count-largest-group

use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut cnt = HashMap::new();

        for mut i in 1..=n {
            let mut s = 0;
            while i > 0 {
                s += i % 10;
                i /= 10;
            }
            *cnt.entry(s).or_default() += 1_i32;
        }

        let ma: i32 = cnt.values().max().copied().unwrap();

        cnt.values().filter(|&&s| s == ma).count() as i32
    }
}
