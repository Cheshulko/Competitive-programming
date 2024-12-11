// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation

struct Solution {}

impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let max = 2 + k + *nums.iter().max().unwrap() as usize;
        let mut pref = vec![0; 2 * max];

        for x in nums.into_iter() {
            pref[max + x as usize - k] += 1;
            pref[max + x as usize + k + 1] -= 1;
        }

        let mut ans = pref[0];
        for i in 1..2 * max {
            pref[i] += pref[i - 1];
            ans = ans.max(pref[i]);
        }

        ans as i32
    }
}