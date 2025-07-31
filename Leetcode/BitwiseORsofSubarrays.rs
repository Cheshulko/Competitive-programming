// https://leetcode.com/problems/bitwise-ors-of-subarrays

struct Solution {}

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        const B: usize = 32;

        let n = arr.len();

        let mut right_most = [-1; B];
        let mut pref = vec![[0; B]; n + 1];

        let mut ans = HashSet::new();

        let mut cur = 0;
        for (i, num) in arr.into_iter().enumerate() {
            cur |= num;
            ans.insert(cur);

            for b in 0..B {
                let has_bit = ((1 << b) & num) > 0;
                pref[i + 1][b] += pref[i][b] + (has_bit as usize);
            }

            for b in 0..B {
                let j = (right_most[b] + 1) as usize;

                let mut cur = 0;
                for b2 in 0..B {
                    if pref[i + 1][b2] - pref[j][b2] > 0 {
                        cur |= 1 << b2;
                    }
                }
                ans.insert(cur);
            }

            for b in 0..B {
                let has_bit = ((1 << b) & num) > 0;
                if has_bit {
                    right_most[b] = i as i32;
                }
            }
        }

        ans.len() as i32
    }
}
