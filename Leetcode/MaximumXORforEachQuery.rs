// https://leetcode.com/problems/maximum-xor-for-each-query

struct Solution {}

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![];

        let mut pref = 0;
        for i in 0..n {
            pref ^= nums[i];
        }

        for i in (0..n).rev() {
            let mut k = 0;
            for b in (0..maximum_bit).rev() {
                k ^= (1 << b) ^ (pref & (1 << b))
            }
            ans.push(k);
            pref ^= nums[i];
        }

        ans
    }
}
