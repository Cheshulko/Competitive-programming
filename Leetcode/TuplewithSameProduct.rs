// https://leetcode.com/problems/tuple-with-same-product

use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            if a == 0 {
                return b;
            }
            if b == 0 {
                return a;
            }

            while a != 0 {
                if a < b {
                    std::mem::swap(&mut a, &mut b);
                }
                a %= b;
            }
            b
        }

        let n = nums.len();

        let mut ratio = HashMap::<(i32, i32), usize>::new();
        let mut cnt = vec![HashMap::<(i32, i32), usize>::new(); n];
        for i in 0..n {
            for j in i + 1..n {
                let (a, b) = (nums[i], nums[j]);
                let g = gcd(a, b);
                {
                    let r = (a / g, b / g);
                    *ratio.entry(r).or_default() += 1;
                    *cnt[i].entry(r).or_default() += 1;
                }
                {
                    let r = (b / g, a / g);
                    *ratio.entry(r).or_default() += 1;
                    *cnt[j].entry(r).or_default() += 1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let (a, b) = (nums[i], nums[j]);
                let g = gcd(a, b);
                let r = (b / g, a / g);
                let r0 = (a / g, b / g);

                let mut x = ratio.get(&r).cloned().unwrap_or(0);
                x -= cnt[i].get(&r).copied().unwrap_or(0);
                x -= cnt[j].get(&r).copied().unwrap_or(0);
                x -= cnt[i].get(&r0).copied().unwrap_or(0);
                x -= cnt[j].get(&r0).copied().unwrap_or(0);
                x += 1;

                ans += x;
            }
        }

        ans as i32
    }
}
