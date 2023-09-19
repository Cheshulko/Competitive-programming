// https://leetcode.com/problems/find-the-duplicate-number

struct Solution {}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut res = 0;
        for i in 0..32 {
            let bit = 1 << i;
            let mut cnt_have = 0;
            let mut cnt_must = 0;
            for j in 0..nums.len() {
                cnt_must += (j & bit > 0) as i32;
                cnt_have += (nums[j] & bit > 0) as i32;
            }
            if cnt_have > cnt_must {
                res += bit;
            }
        }
        res as i32
    }
}
