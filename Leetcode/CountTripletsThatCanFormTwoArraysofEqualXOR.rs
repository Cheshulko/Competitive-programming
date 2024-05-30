// https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut hm = HashMap::<i32, usize>::new();

        let mut x = vec![0; n + 1];
        for i in 1..=n {
            x[i] = x[i - 1] ^ arr[i - 1];
        }

        let mut ans = 0;
        for i in 1..=n {
            for j in (i + 1)..=n {
                for k in j..=n {
                    let xx = x[i - 1] ^ x[j - 1];
                    let yy = x[j - 1] ^ x[k];

                    ans += (xx == yy) as i32;
                }
            }
        }

        ans as i32
    }
}
