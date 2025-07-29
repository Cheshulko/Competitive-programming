// https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or

struct Solution {}

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        const B: usize = 32;

        let n = nums.len();

        let mut bit_indxs = vec![vec![]; B];
        for (i, num) in nums.into_iter().enumerate().rev() {
            for b in 0..B {
                let has_bit = (num & (1 << b)) > 0;
                if has_bit {
                    bit_indxs[b].push(i);
                }
            }
        }

        let mut ans = vec![0; n];
        for i in 0..n {
            let mut ind = i;
            for b in 0..B {
                if let Some(&bi) = bit_indxs[b].last() {
                    ind = ind.max(bi);

                    if bi == i {
                        bit_indxs[b].pop();
                    }
                }
            }

            ans[i] = (ind - i + 1) as i32;
        }

        ans
    }
}
